use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("no audio input provided")]
    NoInput,

    #[error("input not found: {path}")]
    InputNotFound { path: String },

    #[error("audio decode failed: {0}")]
    AudioDecode(#[source] anyhow::Error),

    #[error("unsupported audio format: {format}")]
    UnsupportedFormat { format: String },

    #[error("video extraction failed: {path}: {ffmpeg_stderr}")]
    VideoExtractionFailed { path: String, ffmpeg_stderr: String },

    #[error("ffmpeg not found in PATH: install via `brew install ffmpeg` or set --ffmpeg-binary")]
    FfmpegNotFound,

    #[error("unsupported video format: {format}")]
    UnsupportedVideoFormat { format: String },

    #[error("model not found: {name}")]
    ModelNotFound { name: String },

    #[error("model download failed: {0}")]
    ModelDownload(#[source] anyhow::Error),

    #[error("whisper inference failed: {0}")]
    WhisperInference(String),

    #[error("unsupported platform: macOS with Apple Silicon required")]
    UnsupportedPlatform,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("configuration error: {0}")]
    Config(String),
}

impl Error {
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::NoInput => 64,
            Self::InputNotFound { .. } => 66,
            Self::AudioDecode(_) => 65,
            Self::UnsupportedFormat { .. } => 65,
            Self::VideoExtractionFailed { .. } => 65,
            Self::FfmpegNotFound => 69,
            Self::UnsupportedVideoFormat { .. } => 65,
            Self::ModelNotFound { .. } => 78,
            Self::ModelDownload(_) => 69,
            Self::WhisperInference(_) => 70,
            Self::UnsupportedPlatform => 69,
            Self::Io(_) => 74,
            Self::Config(_) => 78,
        }
    }

    pub fn to_exit_code(&self) -> ExitCode {
        ExitCode::from(self.exit_code())
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::NoInput => "usage",
            Self::InputNotFound { .. } => "input",
            Self::AudioDecode(_)
            | Self::UnsupportedFormat { .. }
            | Self::VideoExtractionFailed { .. }
            | Self::UnsupportedVideoFormat { .. } => "data",
            Self::FfmpegNotFound => "service",
            Self::ModelNotFound { .. } | Self::Config(_) => "config",
            Self::ModelDownload(_) | Self::UnsupportedPlatform => "service",
            Self::WhisperInference(_) => "internal",
            Self::Io(_) => "io",
        }
    }

    pub fn retryable(&self) -> bool {
        matches!(self, Self::ModelDownload(_))
    }

    pub fn retry_after_ms(&self) -> Option<u64> {
        match self {
            Self::ModelDownload(_) => Some(2000),
            _ => None,
        }
    }

    pub fn hint(&self) -> Option<&'static str> {
        match self {
            Self::NoInput => Some("provide audio file(s) as arguments or pipe via stdin"),
            Self::InputNotFound { .. } => Some("check the file path and try again"),
            Self::AudioDecode(_) => {
                Some("verify the file is a valid audio format (ogg, mp3, wav, flac)")
            }
            Self::UnsupportedFormat { .. } => Some("use --input-format to force a specific codec"),
            Self::VideoExtractionFailed { .. } => {
                Some("ffmpeg failed to extract audio; check codec and --ffmpeg-binary")
            }
            Self::FfmpegNotFound => {
                Some("install ffmpeg via `brew install ffmpeg` or set --ffmpeg-binary")
            }
            Self::UnsupportedVideoFormat { .. } => Some("supported: mp4, mov, m4v, mkv, webm, avi"),
            Self::ModelNotFound { .. } => {
                Some("run 'whisper-macos-cli models list' to see available models")
            }
            Self::ModelDownload(_) => Some("check network connectivity and retry"),
            Self::WhisperInference(_) => Some("try a smaller model with --model base"),
            Self::UnsupportedPlatform => Some("this CLI requires macOS with Apple Silicon (M1+)"),
            Self::Io(_) => None,
            Self::Config(_) => Some("run 'whisper-macos-cli doctor' to diagnose"),
        }
    }

    pub fn docs_url(&self) -> &'static str {
        match self {
            Self::NoInput => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/AGENTS.md#contract"
            }
            Self::InputNotFound { .. } => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md"
            }
            Self::AudioDecode(_) => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md#audio-decode"
            }
            Self::UnsupportedFormat { .. } => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md#unsupported-format"
            }
            Self::VideoExtractionFailed { .. } => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/VIDEO-EXTRACTION.md"
            }
            Self::FfmpegNotFound => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/VIDEO-EXTRACTION.md#ffmpeg-not-found"
            }
            Self::UnsupportedVideoFormat { .. } => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/VIDEO-EXTRACTION.md#supported-formats"
            }
            Self::ModelNotFound { .. } => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/AGENTS.md#model-management"
            }
            Self::ModelDownload(_) => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md#model-download"
            }
            Self::WhisperInference(_) => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md#inference"
            }
            Self::UnsupportedPlatform => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/README.md#platform-requirements"
            }
            Self::Io(_) => {
                "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md"
            }
            Self::Config(_) => {
                "https://github.com/daniloteixeira/Dropbox/ai/dev/rust/macos/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md"
            }
        }
    }

    pub fn to_json(&self, correlation_id: &str) -> serde_json::Value {
        serde_json::json!({
            "schema_version": env!("CARGO_PKG_VERSION"),
            "error": true,
            "code": self.exit_code(),
            "message": self.to_string(),
            "category": self.category(),
            "retryable": self.retryable(),
            "retry_after_ms": self.retry_after_ms(),
            "hint": self.hint(),
            "docs_url": self.docs_url(),
            "correlation_id": correlation_id,
        })
    }
}

/// Build a stable JSON error envelope from any [`Error`].
///
/// # Example
///
/// ```
/// use whisper_macos_cli::error::Error;
///
/// let err = Error::InputNotFound { path: "missing.ogg".to_string() };
/// let envelope = err.to_json("test-correlation-id");
/// assert_eq!(envelope["error"], true);
/// assert_eq!(envelope["code"], 66);
/// assert_eq!(envelope["category"], "input");
/// assert_eq!(envelope["correlation_id"], "test-correlation-id");
/// assert!(envelope["docs_url"].as_str().unwrap().starts_with("https://"));
/// ```
pub type ErrorEnvelope = serde_json::Value;

#[doc(hidden)]
pub fn _doc_test_compiles() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_exit_code_is_64() {
        assert_eq!(Error::NoInput.exit_code(), 64);
    }

    #[test]
    fn input_not_found_exit_code_is_66() {
        let err = Error::InputNotFound {
            path: "test.mp3".to_string(),
        };
        assert_eq!(err.exit_code(), 66);
    }

    #[test]
    fn model_not_found_exit_code_is_78() {
        let err = Error::ModelNotFound {
            name: "unknown".to_string(),
        };
        assert_eq!(err.exit_code(), 78);
    }

    #[test]
    fn error_json_contains_all_required_fields() {
        let err = Error::ModelDownload(anyhow::anyhow!("HTTP 503"));
        let json = err.to_json("test-corr-id");
        assert_eq!(json["error"], true);
        assert!(json["code"].is_number());
        assert!(json["message"].is_string());
        assert!(json["category"].is_string());
        assert!(json["retryable"].is_boolean());
        assert!(json["retry_after_ms"].is_number());
        assert!(json["docs_url"].is_string());
        assert_eq!(json["correlation_id"], "test-corr-id");
    }

    #[test]
    fn error_json_uses_pkg_version_for_schema_version() {
        let err = Error::NoInput;
        let json = err.to_json("any");
        assert_eq!(json["schema_version"], env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn category_assignments_are_correct() {
        assert_eq!(Error::NoInput.category(), "usage");
        assert_eq!(
            Error::InputNotFound { path: "x".into() }.category(),
            "input"
        );
        assert_eq!(
            Error::UnsupportedFormat { format: "x".into() }.category(),
            "data"
        );
        assert_eq!(
            Error::ModelNotFound { name: "x".into() }.category(),
            "config"
        );
        assert_eq!(
            Error::ModelDownload(anyhow::anyhow!("x")).category(),
            "service"
        );
        assert_eq!(Error::UnsupportedPlatform.category(), "service");
        assert_eq!(Error::WhisperInference("x".into()).category(), "internal");
        assert_eq!(Error::Config("x".into()).category(), "config");
    }

    #[test]
    fn retryable_only_model_download() {
        assert!(Error::ModelDownload(anyhow::anyhow!("x")).retryable());
        assert!(!Error::NoInput.retryable());
        assert!(!Error::InputNotFound { path: "x".into() }.retryable());
        assert!(!Error::UnsupportedFormat { format: "x".into() }.retryable());
        assert!(!Error::ModelNotFound { name: "x".into() }.retryable());
        assert!(!Error::WhisperInference("x".into()).retryable());
        assert!(!Error::Config("x".into()).retryable());
    }

    #[test]
    fn retry_after_ms_only_for_model_download() {
        assert_eq!(
            Error::ModelDownload(anyhow::anyhow!("x")).retry_after_ms(),
            Some(2000)
        );
        assert_eq!(Error::NoInput.retry_after_ms(), None);
        assert_eq!(Error::Config("x".into()).retry_after_ms(), None);
    }

    #[test]
    fn hint_present_for_recoverable_errors() {
        assert!(Error::NoInput.hint().is_some());
        assert!(Error::InputNotFound { path: "x".into() }.hint().is_some());
        assert!(Error::ModelNotFound { name: "x".into() }.hint().is_some());
    }

    #[test]
    fn docs_url_is_full_github_url() {
        for err in [
            Error::NoInput,
            Error::InputNotFound { path: "x".into() },
            Error::ModelNotFound { name: "x".into() },
            Error::UnsupportedPlatform,
            Error::Config("x".into()),
        ] {
            let url = err.docs_url();
            assert!(url.starts_with("https://"), "{url} should be https");
        }
    }

    #[test]
    fn error_display_messages_are_lowercase() {
        let msgs = [
            Error::NoInput.to_string(),
            Error::InputNotFound { path: "x".into() }.to_string(),
            Error::UnsupportedFormat { format: "x".into() }.to_string(),
            Error::ModelNotFound { name: "x".into() }.to_string(),
            Error::WhisperInference("x".into()).to_string(),
            Error::UnsupportedPlatform.to_string(),
        ];
        for msg in msgs {
            assert!(
                !msg.ends_with('.'),
                "msg `{msg}` should not end with period"
            );
        }
    }

    #[test]
    fn exit_codes_match_sysexits_h() {
        assert_eq!(Error::NoInput.exit_code(), 64);
        assert_eq!(Error::InputNotFound { path: "x".into() }.exit_code(), 66);
        assert_eq!(Error::AudioDecode(anyhow::anyhow!("x")).exit_code(), 65);
        assert_eq!(
            Error::UnsupportedFormat { format: "x".into() }.exit_code(),
            65
        );
        assert_eq!(Error::ModelNotFound { name: "x".into() }.exit_code(), 78);
        assert_eq!(Error::ModelDownload(anyhow::anyhow!("x")).exit_code(), 69);
        assert_eq!(Error::WhisperInference("x".into()).exit_code(), 70);
        assert_eq!(Error::UnsupportedPlatform.exit_code(), 69);
        assert_eq!(Error::Config("x".into()).exit_code(), 78);
    }

    #[test]
    fn video_extraction_failed_exit_code_is_65() {
        let err = Error::VideoExtractionFailed {
            path: "video.mp4".into(),
            ffmpeg_stderr: "Invalid data found".into(),
        };
        assert_eq!(err.exit_code(), 65);
        assert_eq!(err.category(), "data");
        assert!(!err.retryable());
    }

    #[test]
    fn ffmpeg_not_found_exit_code_is_69() {
        assert_eq!(Error::FfmpegNotFound.exit_code(), 69);
        assert_eq!(Error::FfmpegNotFound.category(), "service");
        assert!(!Error::FfmpegNotFound.retryable());
        assert!(
            Error::FfmpegNotFound
                .hint()
                .unwrap()
                .contains("brew install ffmpeg")
        );
    }

    #[test]
    fn unsupported_video_format_exit_code_is_65() {
        let err = Error::UnsupportedVideoFormat {
            format: "wmv".into(),
        };
        assert_eq!(err.exit_code(), 65);
        assert_eq!(err.category(), "data");
        assert!(err.hint().unwrap().contains("mp4"));
    }

    #[test]
    fn video_errors_have_video_docs_url() {
        let err = Error::VideoExtractionFailed {
            path: "x".into(),
            ffmpeg_stderr: "y".into(),
        };
        assert!(err.docs_url().contains("VIDEO-EXTRACTION"));
        assert!(
            Error::FfmpegNotFound
                .docs_url()
                .contains("VIDEO-EXTRACTION")
        );
        assert!(
            Error::UnsupportedVideoFormat { format: "x".into() }
                .docs_url()
                .contains("VIDEO-EXTRACTION")
        );
    }
}
