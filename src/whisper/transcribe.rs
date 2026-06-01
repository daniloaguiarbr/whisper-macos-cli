use crate::output::Segment;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

pub fn transcribe_chunk(
    ctx: &WhisperContext,
    audio_f32: &[f32],
    language: &str,
    beam_size: i32,
    with_timestamps: bool,
) -> Result<(String, Vec<Segment>), crate::error::Error> {
    let mut params = FullParams::new(SamplingStrategy::BeamSearch {
        beam_size,
        patience: -1.0,
    });

    let lang = if language == "auto" {
        None
    } else {
        Some(language)
    };
    params.set_language(lang);
    params.set_translate(false);
    params.set_no_context(true);
    params.set_temperature(0.0);
    params.set_temperature_inc(0.2);
    params.set_entropy_thold(2.4);
    params.set_logprob_thold(-1.0);
    params.set_suppress_blank(true);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    let mut state = ctx
        .create_state()
        .map_err(|e| crate::error::Error::WhisperInference(e.to_string()))?;

    state
        .full(params, audio_f32)
        .map_err(|e| crate::error::Error::WhisperInference(e.to_string()))?;

    let n_segments = state.full_n_segments();

    let mut full_text = String::new();
    let mut segments = Vec::new();

    for i in 0..n_segments {
        let seg = state.get_segment(i).ok_or_else(|| {
            crate::error::Error::WhisperInference(format!("segment {i} out of bounds"))
        })?;

        let text = seg
            .to_str_lossy()
            .map_err(|e| crate::error::Error::WhisperInference(e.to_string()))?;

        full_text.push_str(&text);

        if with_timestamps {
            let t0 = seg.start_timestamp();
            let t1 = seg.end_timestamp();

            segments.push(Segment {
                start: t0 as f64 / 100.0,
                end: t1 as f64 / 100.0,
                text: text.to_string(),
            });
        }
    }

    Ok((full_text, segments))
}
