pub fn detect_language() -> &'static str {
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string());
    let lang_code = locale.split(['_', '-']).next().unwrap_or("en");
    map_to_whisper_code(lang_code)
}

pub fn map_to_whisper_code(code: &str) -> &'static str {
    match code {
        "af" => "af",
        "am" => "am",
        "ar" => "ar",
        "as" => "as",
        "az" => "az",
        "ba" => "ba",
        "be" => "be",
        "bg" => "bg",
        "bn" => "bn",
        "bo" => "bo",
        "br" => "br",
        "bs" => "bs",
        "ca" => "ca",
        "cs" => "cs",
        "cy" => "cy",
        "da" => "da",
        "de" => "de",
        "el" => "el",
        "en" => "en",
        "es" => "es",
        "et" => "et",
        "eu" => "eu",
        "fa" => "fa",
        "fi" => "fi",
        "fo" => "fo",
        "fr" => "fr",
        "gl" => "gl",
        "gu" => "gu",
        "ha" => "ha",
        "haw" => "haw",
        "he" => "he",
        "hi" => "hi",
        "hr" => "hr",
        "ht" => "ht",
        "hu" => "hu",
        "hy" => "hy",
        "id" => "id",
        "is" => "is",
        "it" => "it",
        "ja" => "ja",
        "jw" => "jw",
        "ka" => "ka",
        "kk" => "kk",
        "km" => "km",
        "kn" => "kn",
        "ko" => "ko",
        "la" => "la",
        "lb" => "lb",
        "ln" => "ln",
        "lo" => "lo",
        "lt" => "lt",
        "lv" => "lv",
        "mg" => "mg",
        "mi" => "mi",
        "mk" => "mk",
        "ml" => "ml",
        "mn" => "mn",
        "mr" => "mr",
        "ms" => "ms",
        "mt" => "mt",
        "my" => "my",
        "ne" => "ne",
        "nl" => "nl",
        "nn" => "nn",
        "no" => "no",
        "oc" => "oc",
        "pa" => "pa",
        "pl" => "pl",
        "ps" => "ps",
        "pt" => "pt",
        "ro" => "ro",
        "ru" => "ru",
        "sa" => "sa",
        "sd" => "sd",
        "si" => "si",
        "sk" => "sk",
        "sl" => "sl",
        "sn" => "sn",
        "so" => "so",
        "sq" => "sq",
        "sr" => "sr",
        "su" => "su",
        "sv" => "sv",
        "sw" => "sw",
        "ta" => "ta",
        "te" => "te",
        "tg" => "tg",
        "th" => "th",
        "tk" => "tk",
        "tl" => "tl",
        "tr" => "tr",
        "tt" => "tt",
        "uk" => "uk",
        "ur" => "ur",
        "uz" => "uz",
        "vi" => "vi",
        "yi" => "yi",
        "yo" => "yo",
        "zh" => "zh",
        _ => "en",
    }
}

pub fn resolve_language(cli_lang: Option<&str>) -> (&'static str, &'static str) {
    match cli_lang {
        Some("auto") => ("auto", "whisper_auto"),
        Some(lang) => {
            let resolved = map_to_whisper_code(lang);
            if resolved == "en" && lang != "en" {
                tracing::warn!(
                    requested = lang,
                    resolved = "en",
                    "unknown language code, falling back to English"
                );
            }
            (resolved, "cli")
        }
        None => (detect_language(), "os_locale"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_language_explicit_pt() {
        let (lang, source) = resolve_language(Some("pt"));
        assert_eq!(lang, "pt");
        assert_eq!(source, "cli");
    }

    #[test]
    fn resolve_language_auto() {
        let (lang, source) = resolve_language(Some("auto"));
        assert_eq!(lang, "auto");
        assert_eq!(source, "whisper_auto");
    }

    #[test]
    fn resolve_language_none_returns_os_locale() {
        let (lang, source) = resolve_language(None);
        assert!(!lang.is_empty());
        assert_eq!(source, "os_locale");
    }

    #[test]
    fn map_to_whisper_code_returns_known_codes() {
        for code in &["pt", "en", "es", "fr", "de", "it", "ja", "zh", "ru", "ar"] {
            assert_eq!(
                map_to_whisper_code(code),
                *code,
                "code {code} should be identity"
            );
        }
    }

    #[test]
    fn map_to_whisper_code_falls_back_to_english_for_unknown() {
        for code in &["xx", "klingon", "elvish", "na", ""] {
            assert_eq!(
                map_to_whisper_code(code),
                "en",
                "unknown {code} should fallback to en"
            );
        }
    }

    #[test]
    fn map_to_whisper_code_returns_non_empty_for_all_iso_639_1() {
        let codes = [
            "aa", "ab", "af", "ak", "am", "ar", "as", "ay", "az", "ba", "be", "bg", "bh", "bi",
            "bm", "bn", "bo", "br", "bs", "ca", "ce", "ch", "co", "cr", "cs", "cu", "cv", "cy",
            "da", "de", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff", "fi",
            "fj", "fo", "fr", "fy", "ga", "gd", "gl", "gn", "gu", "gv", "ha", "he", "hi", "ho",
            "hr", "ht", "hu", "hy", "hz", "ia", "id", "ie", "ig", "ii", "ik", "io", "is", "it",
            "iu", "ja", "jv", "ka", "kg", "ki", "kj", "kk", "kl", "km", "kn", "ko", "kr", "ks",
            "ku", "kv", "kw", "ky", "la", "lb", "lg", "li", "ln", "lo", "lt", "lu", "lv", "mg",
            "mh", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "na", "nb", "nd", "ne", "ng",
            "nl", "nn", "no", "nr", "nv", "ny", "oc", "oj", "om", "or", "os", "pa", "pi", "pl",
            "ps", "pt", "qu", "rm", "rn", "ro", "ru", "rw", "sa", "sc", "sd", "se", "sg", "si",
            "sk", "sl", "sm", "sn", "so", "sq", "sr", "ss", "st", "su", "sv", "sw", "ta", "te",
            "tg", "th", "ti", "tk", "tl", "tn", "to", "tr", "ts", "tt", "tw", "ty", "ug", "uk",
            "ur", "uz", "ve", "vi", "vo", "wa", "wo", "xh", "yi", "yo", "za", "zh", "zu",
        ];
        for code in &codes {
            let result = map_to_whisper_code(code);
            assert!(!result.is_empty(), "code {code} should not return empty");
        }
    }

    #[test]
    fn map_to_whisper_code_handles_short_codes() {
        assert_eq!(map_to_whisper_code("pt"), "pt");
        assert_eq!(map_to_whisper_code("en"), "en");
        assert_eq!(map_to_whisper_code("es"), "es");
        assert_eq!(map_to_whisper_code("zh"), "zh");
        assert_eq!(map_to_whisper_code("ar"), "ar");
        assert_eq!(map_to_whisper_code("ja"), "ja");
    }
}
