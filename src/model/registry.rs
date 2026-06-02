pub struct ModelInfo {
    pub name: &'static str,
    pub filename: &'static str,
    pub url: &'static str,
    pub size_bytes: u64,
    pub description: &'static str,
    pub min_size_bytes: u64,
}

static MODELS: &[ModelInfo] = &[
    ModelInfo {
        name: "tiny",
        filename: "ggml-tiny.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
        size_bytes: 75_687_065,
        min_size_bytes: 70_000_000,
        description: "Fastest, lowest accuracy",
    },
    ModelInfo {
        name: "base",
        filename: "ggml-base.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
        size_bytes: 142_356_480,
        min_size_bytes: 135_000_000,
        description: "Fast, basic accuracy",
    },
    ModelInfo {
        name: "small",
        filename: "ggml-small.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        size_bytes: 466_041_792,
        min_size_bytes: 440_000_000,
        description: "Balanced speed/accuracy",
    },
    ModelInfo {
        name: "medium",
        filename: "ggml-medium.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
        size_bytes: 1_524_630_880,
        min_size_bytes: 1_400_000_000,
        description: "High accuracy",
    },
    ModelInfo {
        name: "large-v3",
        filename: "ggml-large-v3.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin",
        size_bytes: 3_095_033_856,
        min_size_bytes: 2_900_000_000,
        description: "Maximum accuracy (default)",
    },
];

pub fn get_model(name: &str) -> Option<&'static ModelInfo> {
    MODELS.iter().find(|m| m.name == name)
}

pub fn default_model() -> &'static ModelInfo {
    match get_model("large-v3") {
        Some(m) => m,
        None => &MODELS[MODELS.len() - 1],
    }
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

    #[test]
    fn min_size_is_less_than_total_size() {
        for m in all_models() {
            assert!(
                m.min_size_bytes < m.size_bytes,
                "min_size must be smaller than expected size for {}",
                m.name
            );
        }
    }
}
