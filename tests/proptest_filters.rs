use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn filter_hallucinations_preserves_alphanumeric_lines(
        text in "[A-Za-z][A-Za-z0-9 .!?\\n]{0,199}"
    ) {
        let result = whisper_macos_cli::audio::vad::filter_hallucinations(&text);
        for word in text.split_whitespace() {
            prop_assert!(result.contains(word), "word `{word}` missing from result");
        }
        prop_assert!(!result.contains("legenda"));
        prop_assert!(!result.contains("amara"));
        prop_assert!(!result.contains("inscreva"));
        prop_assert!(!result.contains("www."));
        prop_assert!(!result.contains("transcri"));
    }

    #[test]
    fn filter_hallucinations_strips_all_known_markers(
        marker in prop_oneof![
            Just("legendas por comunidade"),
            Just("LEGENDA POR AMARA"),
            Just("inscreva-se"),
            Just("Inscreva-Se"),
            Just("amara.org"),
            Just("www.example.com"),
            Just("transcrição por fulano"),
            Just("transcrição e legendas"),
        ]
    ) {
        let text = format!("Hello world\n{marker}\nGoodbye world");
        let result = whisper_macos_cli::audio::vad::filter_hallucinations(&text);
        prop_assert!(!result.contains("legenda"), "result retained 'legenda': {result}");
        prop_assert!(!result.contains("amara"), "result retained 'amara': {result}");
        prop_assert!(!result.contains("inscreva"), "result retained 'inscreva': {result}");
        prop_assert!(!result.contains("www."), "result retained 'www.': {result}");
        prop_assert!(!result.contains("transcri"), "result retained 'transcri': {result}");
        prop_assert!(result.contains("Hello world"));
        prop_assert!(result.contains("Goodbye world"));
    }

    #[test]
    fn collapse_consecutive_repeats_collapses_to_unique(
        lines in proptest::collection::vec("[A-Za-z]{1,20}", 1..20)
    ) {
        let input: String = lines.iter().map(|l| format!("{l}\n")).collect();
        let result = whisper_macos_cli::audio::vad::collapse_consecutive_repeats(&input);
        let result_lines: Vec<&str> = result.lines().collect();
        for window in result_lines.windows(2) {
            prop_assert_ne!(window[0], window[1], "consecutive duplicates remain: {:?}", window);
        }
    }

    #[test]
    fn filter_hallucinations_preserves_unicode_text(
        text in "[\\p{L}\\p{N} .,!?\\n]{0,300}"
    ) {
        let result = whisper_macos_cli::audio::vad::filter_hallucinations(&text);
        prop_assert!(!result.contains("legenda"));
        prop_assert!(!result.contains("amara"));
        prop_assert!(!result.contains("inscreva"));
        prop_assert!(!result.contains("www."));
        prop_assert!(!result.contains("transcri"));
    }

    #[test]
    fn collapse_consecutive_repeats_preserves_unicode_text(
        lines in proptest::collection::vec("[\\p{L}\\p{N} ]{1,20}", 1..10)
    ) {
        let input: String = lines.iter().map(|l| format!("{l}\n")).collect();
        let result = whisper_macos_cli::audio::vad::collapse_consecutive_repeats(&input);
        let result_lines: Vec<&str> = result.lines().collect();
        for window in result_lines.windows(2) {
            prop_assert_ne!(window[0], window[1], "consecutive duplicates remain: {:?}", window);
        }
    }
}
