pub struct ModelInfo {
    pub name: &'static str,
    pub filename: &'static str,
    pub url: &'static str,
    pub size_bytes: u64,
    pub description: &'static str,
}

static MODELS: &[ModelInfo] = &[
    ModelInfo {
        name: "tiny",
        filename: "ggml-tiny.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
        size_bytes: 75_000_000,
        description: "Fastest, lowest accuracy",
    },
    ModelInfo {
        name: "base",
        filename: "ggml-base.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
        size_bytes: 142_000_000,
        description: "Fast, basic accuracy",
    },
    ModelInfo {
        name: "small",
        filename: "ggml-small.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        size_bytes: 466_000_000,
        description: "Balanced speed/accuracy",
    },
    ModelInfo {
        name: "medium",
        filename: "ggml-medium.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
        size_bytes: 1_500_000_000,
        description: "High accuracy",
    },
    ModelInfo {
        name: "large-v3",
        filename: "ggml-large-v3.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin",
        size_bytes: 3_095_033_856,
        description: "Maximum accuracy (default)",
    },
];

pub fn get_model(name: &str) -> Option<&'static ModelInfo> {
    MODELS.iter().find(|m| m.name == name)
}

pub fn default_model() -> &'static ModelInfo {
    get_model("large-v3").expect("large-v3 must be present in MODELS")
}

pub fn all_models() -> &'static [ModelInfo] {
    MODELS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_is_large_v3() {
        assert_eq!(default_model().name, "large-v3");
    }

    #[test]
    fn get_model_tiny_returns_some() {
        assert!(get_model("tiny").is_some());
    }

    #[test]
    fn get_model_nonexistent_returns_none() {
        assert!(get_model("nonexistent").is_none());
    }

    #[test]
    fn all_models_returns_five() {
        assert_eq!(all_models().len(), 5);
    }
}
