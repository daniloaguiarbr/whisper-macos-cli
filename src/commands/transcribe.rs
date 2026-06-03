use std::path::Path;
use std::time::Instant;

use unicode_normalization::UnicodeNormalization;
use whisper_rs::WhisperContext;

use crate::audio::{decode, resample, vad};
use crate::cli::TranscribeArgs;
use crate::error::Error;
use crate::model::{download, registry, storage};
use crate::output::{self, TranscriptionResult};
use crate::video::ffmpeg::RealFfmpeg;
use crate::whisper::{context, transcribe};

const MAX_DURATION_SECONDS: f64 = 24.0 * 3600.0;

pub fn run(
    args: &TranscribeArgs,
    language: &str,
    language_source: &str,
    correlation_id: &str,
) -> Result<(), Error> {
    if args.dry_run {
        tracing::info!(
            dry_run = true,
            "dry run requested — no transcription performed"
        );
        let value = serde_json::json!({
            "schema_version": env!("CARGO_PKG_VERSION"),
            "correlation_id": correlation_id,
            "dry_run": true,
            "would_transcribe": {
                "files": args.files.len(),
                "model": args.model.as_str(),
                "language": language,
                "beam_size": args.beam_size,
            }
        });
        output::write_json_value(&value).map_err(Error::Io)?;
        return Ok(());
    }

    let model_info =
        registry::get_model(args.model.as_str()).ok_or_else(|| Error::ModelNotFound {
            name: args.model.as_str().to_string(),
        })?;

    if !storage::is_model_downloaded(model_info)? {
        tracing::info!(
            model = model_info.name,
            "model not found locally, downloading"
        );
        let dest = storage::model_path(model_info)?;
        download::download_model(
            model_info.url,
            &dest,
            model_info.size_bytes,
            model_info.min_size_bytes,
        )?;
    }

    let model_path = storage::model_path(model_info)?;
    tracing::info!(model = model_info.name, "loading whisper model");
    let ctx = context::load_model(&model_path)?;

    let ndjson = args.is_ndjson();

    if args.files.is_empty() {
        let result = transcribe_source(
            &ctx,
            None,
            args,
            language,
            language_source,
            model_info.name,
            correlation_id,
        )?;
        emit_result(&result, ndjson)?;
        if ndjson {
            emit_summary(correlation_id, 1, 0)?;
        }
    } else if args.files.len() == 1 || args.concurrency <= 1 {
        let mut errors = 0u64;
        for file_path in &args.files {
            match transcribe_source(
                &ctx,
                Some(file_path),
                args,
                language,
                language_source,
                model_info.name,
                correlation_id,
            ) {
                Ok(result) => emit_result(&result, ndjson)?,
                Err(e) => {
                    errors += 1;
                    if ndjson {
                        let _ = output::write_error(&e, correlation_id);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        if ndjson {
            emit_summary(correlation_id, args.files.len() as u64, errors)?;
        }
    } else {
        let mut errors = 0u64;
        let total = args.files.len() as u64;
        for chunk in args.files.chunks(args.concurrency) {
            let results: Vec<_> = std::thread::scope(|s| {
                let handles: Vec<_> = chunk
                    .iter()
                    .map(|file_path| {
                        s.spawn(|| {
                            transcribe_source(
                                &ctx,
                                Some(file_path),
                                args,
                                language,
                                language_source,
                                model_info.name,
                                correlation_id,
                            )
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|h| match h.join() {
                        Ok(result) => result,
                        Err(_) => Err(Error::WhisperInference(
                            "thread panicked during transcription".into(),
                        )),
                    })
                    .collect()
            });
            for result in results {
                match result {
                    Ok(r) => emit_result(&r, ndjson)?,
                    Err(e) => {
                        errors += 1;
                        if ndjson {
                            let _ = output::write_error(&e, correlation_id);
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }
        if ndjson {
            emit_summary(correlation_id, total, errors)?;
        }
    }

    Ok(())
}

fn transcribe_source(
    ctx: &WhisperContext,
    file_path: Option<&Path>,
    args: &TranscribeArgs,
    language: &str,
    language_source: &str,
    model_name: &str,
    correlation_id: &str,
) -> Result<TranscriptionResult, Error> {
    let start = Instant::now();

    let runner = RealFfmpeg::new(args.ffmpeg_binary.as_str());
    let pcm = match file_path {
        Some(path) => decode::decode_file_with_runner(path, &runner, !args.no_ffmpeg_fallback)?,
        None => decode::decode_stdin(args.input_format.as_deref())?,
    };

    if pcm.duration_seconds() > MAX_DURATION_SECONDS {
        return Err(Error::Config(format!(
            "audio duration {:.1}h exceeds maximum {:.1}h (DoS protection)",
            pcm.duration_seconds() / 3600.0,
            MAX_DURATION_SECONDS / 3600.0
        )));
    }

    let mono = decode::to_mono(&pcm.samples, pcm.channels);
    let resampled = resample::resample_to_16khz(&mono, pcm.sample_rate)?;

    let duration_seconds = resampled.len() as f64 / 16000.0;

    let chunks = vad::detect_speech_segments(&resampled, args.vad_threshold);
    let vad_chunks = chunks.len();

    let mut all_text = String::new();
    let mut all_segments = Vec::new();

    if chunks.is_empty() {
        tracing::warn!("VAD detected no speech — transcribing full audio as fallback");
        let audio_f32 = decode::i16_to_f32(&resampled);
        let (text, segs) = transcribe::transcribe_chunk(
            ctx,
            &audio_f32,
            language,
            args.beam_size,
            args.timestamps,
        )?;
        all_text = text;
        all_segments = segs;
    } else {
        for (idx, (start_sample, end_sample)) in chunks.iter().enumerate() {
            if crate::signal::is_shutdown_requested() {
                return Err(Error::Config("transcription interrupted by signal".into()));
            }
            let chunk_i16 = &resampled[*start_sample..*end_sample];
            let chunk_f32 = decode::i16_to_f32(chunk_i16);

            let chunk_duration = chunk_i16.len() as f32 / 16000.0;
            tracing::info!(
                chunk = idx + 1,
                total = vad_chunks,
                duration_s = %format!("{:.1}", chunk_duration),
                "transcribing chunk"
            );

            let (text, segs) = transcribe::transcribe_chunk(
                ctx,
                &chunk_f32,
                language,
                args.beam_size,
                args.timestamps,
            )?;

            if !text.is_empty() {
                all_text.push_str(&text);
                all_text.push('\n');
            }
            all_segments.extend(segs);
        }
    }

    let filtered = vad::filter_hallucinations(&all_text);
    let final_text = vad::collapse_consecutive_repeats(&filtered);
    let normalized: String = final_text.nfc().collect();

    let file_name = match file_path {
        Some(p) => p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| p.display().to_string()),
        None => "<stdin>".to_string(),
    };

    Ok(TranscriptionResult {
        schema_version: env!("CARGO_PKG_VERSION"),
        correlation_id: correlation_id.to_string(),
        file: file_name,
        language: language.to_string(),
        language_source: language_source.to_string(),
        model: model_name.to_string(),
        duration_seconds,
        text: normalized,
        segments: if args.timestamps {
            Some(all_segments)
        } else {
            None
        },
        vad_chunks,
        processing_time_ms: start.elapsed().as_millis(),
    })
}

fn emit_result(result: &TranscriptionResult, ndjson: bool) -> Result<(), Error> {
    if ndjson {
        output::write_ndjson(result).map_err(Error::Io)
    } else {
        output::write_json(result).map_err(Error::Io)
    }
}

fn emit_summary(correlation_id: &str, total: u64, errors: u64) -> Result<(), Error> {
    let value = serde_json::json!({
        "schema_version": env!("CARGO_PKG_VERSION"),
        "correlation_id": correlation_id,
        "summary": true,
        "total": total,
        "succeeded": total - errors,
        "failed": errors,
    });
    output::write_json_value(&value).map_err(Error::Io)
}
