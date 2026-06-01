use std::path::Path;
use std::time::Instant;

use whisper_rs::WhisperContext;

use crate::audio::{decode, resample, vad};
use crate::cli::TranscribeArgs;
use crate::error::Error;
use crate::model::{download, registry, storage};
use crate::output::{self, TranscriptionResult};
use crate::whisper::{context, transcribe};

pub fn run(args: &TranscribeArgs, language: &str, language_source: &str) -> Result<(), Error> {
    // 1. Resolve model
    let model_info = registry::get_model(&args.model).ok_or_else(|| Error::ModelNotFound {
        name: args.model.clone(),
    })?;

    // 2. Check if model is downloaded, download if not
    if !storage::is_model_downloaded(model_info)? {
        eprintln!(
            "Model '{}' not found locally. Downloading...",
            model_info.name
        );
        let dest = storage::model_path(model_info)?;
        download::download_model(model_info.url, &dest, model_info.size_bytes)?;
    }

    // 3. Load whisper context with Metal GPU
    let model_path = storage::model_path(model_info)?;
    tracing::info!(model = model_info.name, "loading whisper model");
    let ctx = context::load_model(&model_path)?;

    // 4. Process files or stdin
    if args.files.is_empty() {
        let result =
            transcribe_source(&ctx, None, args, language, language_source, model_info.name)?;
        emit_result(&result, args.ndjson)?;
    } else if args.files.len() == 1 || args.concurrency <= 1 {
        for file_path in &args.files {
            match transcribe_source(
                &ctx,
                Some(file_path),
                args,
                language,
                language_source,
                model_info.name,
            ) {
                Ok(result) => emit_result(&result, args.ndjson)?,
                Err(e) => {
                    if args.ndjson {
                        let _ = output::write_error(&e);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    } else {
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
                            )
                        })
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).collect()
            });
            for result in results {
                match result {
                    Ok(r) => emit_result(&r, args.ndjson)?,
                    Err(e) => {
                        if args.ndjson {
                            let _ = output::write_error(&e);
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
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
) -> Result<TranscriptionResult, Error> {
    let start = Instant::now();

    // Decode audio
    let pcm = match file_path {
        Some(path) => decode::decode_file(path)?,
        None => decode::decode_stdin(args.input_format.as_deref())?,
    };

    // Convert to mono then resample to 16 kHz
    let mono = decode::to_mono(&pcm.samples, pcm.channels);
    let resampled = resample::resample_to_16khz(&mono, pcm.sample_rate)?;

    let duration_seconds = resampled.len() as f64 / 16000.0;

    // VAD: split into speech chunks
    let chunks = vad::detect_speech_segments(&resampled, args.vad_threshold);
    let vad_chunks = chunks.len();

    let mut all_text = String::new();
    let mut all_segments = Vec::new();

    if chunks.is_empty() {
        // No speech detected — transcribe the full audio as a fallback
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

    // Post-processing
    let filtered = vad::filter_hallucinations(&all_text);
    let final_text = vad::collapse_consecutive_repeats(&filtered);

    let file_name = match file_path {
        Some(p) => p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| p.display().to_string()),
        None => "<stdin>".to_string(),
    };

    Ok(TranscriptionResult {
        file: file_name,
        language: language.to_string(),
        language_source: language_source.to_string(),
        model: model_name.to_string(),
        duration_seconds,
        text: final_text,
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
