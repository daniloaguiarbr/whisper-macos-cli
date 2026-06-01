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

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": true,
            "code": self.exit_code(),
            "message": self.to_string(),
            "category": self.category(),
            "retryable": self.retryable(),
        })
    }

    fn category(&self) -> &'static str {
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

    fn retryable(&self) -> bool {
        matches!(self, Self::ModelDownload(_))
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
}
