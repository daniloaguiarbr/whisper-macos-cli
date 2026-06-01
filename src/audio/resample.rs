use rubato::{FftFixedIn, Resampler};

const WHISPER_SAMPLE_RATE: u32 = 16000;

pub fn resample_to_16khz(
    samples: &[i16],
    input_rate: u32,
) -> Result<Vec<i16>, crate::error::Error> {
    if input_rate == WHISPER_SAMPLE_RATE {
        return Ok(samples.to_vec());
    }

    let samples_f64: Vec<f64> = samples.iter().map(|&s| s as f64 / 32768.0).collect();

    let chunk_size = 1024;
    let mut resampler = FftFixedIn::<f64>::new(
        input_rate as usize,
        WHISPER_SAMPLE_RATE as usize,
        chunk_size,
        1,
        1,
    )
    .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("resampler init: {e}")))?;

    let mut output_f64: Vec<f64> = Vec::new();
    let mut pos = 0;

    while pos + chunk_size <= samples_f64.len() {
        let chunk = &samples_f64[pos..pos + chunk_size];
        let input = vec![chunk.to_vec()];

        let result = resampler
            .process(&input, None)
            .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("resample: {e}")))?;

        if !result.is_empty() {
            output_f64.extend_from_slice(&result[0]);
        }

        pos += chunk_size;
    }

    if pos < samples_f64.len() {
        let mut last_chunk = samples_f64[pos..].to_vec();
        last_chunk.resize(chunk_size, 0.0);
        let input = vec![last_chunk];

        let result = resampler
            .process(&input, None)
            .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("resample last: {e}")))?;

        if !result.is_empty() {
            let remaining = samples_f64.len() - pos;
            let expected =
                (remaining as f64 * WHISPER_SAMPLE_RATE as f64 / input_rate as f64).ceil() as usize;
            let take = expected.min(result[0].len());
            output_f64.extend_from_slice(&result[0][..take]);
        }
    }

    Ok(output_f64
        .iter()
        .map(|&s| (s.clamp(-1.0, 1.0) * 32767.0) as i16)
        .collect())
}
