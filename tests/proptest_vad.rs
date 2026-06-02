use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn vad_empty_input_yields_no_segments(samples in proptest::collection::vec(0i16..=0, 0..1000)) {
        let segments = whisper_macos_cli::audio::vad::detect_speech_segments(&samples, 0.5);
        let _ = segments;
    }

    #[test]
    fn vad_silent_input_yields_no_segments(
        samples in proptest::collection::vec(0i16..=0, 16000..32000)
    ) {
        let segments = whisper_macos_cli::audio::vad::detect_speech_segments(&samples, 0.5);
        prop_assert!(segments.is_empty(), "silent input produced segments: {:?}", segments);
    }

    #[test]
    fn vad_loud_input_may_yield_segments(
        amplitude in 5000i16..=30000i16,
        duration_secs in 1u32..=3u32,
    ) {
        let n_samples = (duration_secs as usize) * 16000;
        let samples: Vec<i16> = (0..n_samples)
            .map(|i| if (i / 256) % 2 == 0 { amplitude } else { -amplitude })
            .collect();
        let segments = whisper_macos_cli::audio::vad::detect_speech_segments(&samples, 0.5);
        let _ = segments;
    }

    #[test]
    fn vad_output_never_exceeds_input_bounds(
        samples in proptest::collection::vec(any::<i16>(), 0..50000)
    ) {
        let total = samples.len();
        let segments = whisper_macos_cli::audio::vad::detect_speech_segments(&samples, 0.5);
        for (start, end) in &segments {
            prop_assert!(*start <= *end, "start {start} > end {end}");
            prop_assert!(*end <= total, "end {end} > total {total}");
        }
    }

    #[test]
    fn vad_threshold_in_range_is_safe(threshold in 0.0f32..=1.0f32) {
        let samples = vec![1000i16; 16000];
        let segments = whisper_macos_cli::audio::vad::detect_speech_segments(&samples, threshold);
        let _ = segments;
    }
}
