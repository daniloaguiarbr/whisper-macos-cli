use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no audio input provided")]
    NoInput,

    #[error("input not found: {path}")]
    InputNotFound { path: String },

    #[error("audio decode failed: {0}")]
    AudioDecode(#[source] anyhow::Error),

    #[error("unsupported audio format: {format}")]
    UnsupportedFormat { format: String },

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
            Self::AudioDecode(_) | Self::UnsupportedFormat { .. } => "data",
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
}
