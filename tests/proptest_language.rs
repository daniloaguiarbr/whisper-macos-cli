use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn language_code_identity_preserved(code in "[a-z]{2}") {
        let result = whisper_macos_cli::language::detect::map_to_whisper_code(&code);
        let valid_codes = [
            "af", "am", "ar", "as", "az", "ba", "be", "bg", "bn", "bo", "br", "bs", "ca",
            "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fo", "fr",
            "gl", "gu", "ha", "haw", "he", "hi", "hr", "ht", "hu", "hy", "id", "is", "it",
            "ja", "jw", "ka", "kk", "km", "kn", "ko", "la", "lb", "ln", "lo", "lt", "lv",
            "mg", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl", "nn", "no",
            "oc", "pa", "pl", "ps", "pt", "ro", "ru", "sa", "sd", "si", "sk", "sl", "sn",
            "so", "sq", "sr", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
            "tt", "uk", "ur", "uz", "vi", "yi", "yo", "zh",
        ];
        if valid_codes.contains(&code.as_str()) {
            prop_assert_eq!(result, code.as_str(), "known code should be identity");
        } else {
            prop_assert_eq!(result, "en", "unknown code should fallback to en");
        }
    }

    #[test]
    fn language_code_never_empty(code in "[A-Za-z]{0,4}") {
        let result = whisper_macos_cli::language::detect::map_to_whisper_code(&code);
        prop_assert!(!result.is_empty(), "code {code} produced empty result");
    }

    #[test]
    fn language_code_returns_valid_whisper_code(code in "[a-z]{2,3}") {
        let result = whisper_macos_cli::language::detect::map_to_whisper_code(&code);
        prop_assert!(!result.is_empty());
        prop_assert!(result.chars().all(|c| c.is_ascii_lowercase()), "result {result} not lowercase");
        prop_assert!(result.len() <= 5, "result {result} too long");
    }

    #[test]
    fn resolve_language_auto_returns_auto_marker(_dummy in 0..1u8) {
        let (lang, source) = whisper_macos_cli::language::detect::resolve_language(Some("auto"));
        prop_assert_eq!(lang, "auto");
        prop_assert_eq!(source, "whisper_auto");
    }

    #[test]
    fn resolve_language_explicit_sets_cli_source(code in "[a-z]{2,3}") {
        let (_, source) = whisper_macos_cli::language::detect::resolve_language(Some(&code));
        prop_assert_eq!(source, "cli", "explicit code should set source to cli");
    }
}
