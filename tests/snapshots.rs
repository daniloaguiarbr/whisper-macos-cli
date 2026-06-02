use whisper_macos_cli::error::Error;

#[test]
fn snapshot_no_input_error_envelope() {
    let err = Error::NoInput;
    let json = err.to_json("00000000-0000-0000-0000-000000000000");
    insta::assert_json_snapshot!("error_no_input", json);
}

#[test]
fn snapshot_input_not_found_error_envelope() {
    let err = Error::InputNotFound {
        path: "/tmp/missing.ogg".to_string(),
    };
    let json = err.to_json("11111111-1111-1111-1111-111111111111");
    insta::assert_json_snapshot!("error_input_not_found", json);
}

#[test]
fn snapshot_model_not_found_error_envelope() {
    let err = Error::ModelNotFound {
        name: "huge".to_string(),
    };
    let json = err.to_json("22222222-2222-2222-2222-222222222222");
    insta::assert_json_snapshot!("error_model_not_found", json);
}

#[test]
fn snapshot_model_download_retryable_envelope() {
    let err = Error::ModelDownload(anyhow::anyhow!("HTTP 503 Service Unavailable"));
    let json = err.to_json("33333333-3333-3333-3333-333333333333");
    insta::assert_json_snapshot!("error_model_download", json);
}

#[test]
fn snapshot_unsupported_platform_envelope() {
    let err = Error::UnsupportedPlatform;
    let json = err.to_json("44444444-4444-4444-4444-444444444444");
    insta::assert_json_snapshot!("error_unsupported_platform", json);
}

#[test]
fn snapshot_whisper_inference_envelope() {
    let err = Error::WhisperInference("segment 0 out of bounds".to_string());
    let json = err.to_json("55555555-5555-5555-5555-555555555555");
    insta::assert_json_snapshot!("error_whisper_inference", json);
}

#[test]
fn snapshot_config_error_envelope() {
    let err = Error::Config("invalid model path".to_string());
    let json = err.to_json("66666666-6666-6666-6666-666666666666");
    insta::assert_json_snapshot!("error_config", json);
}

#[test]
fn snapshot_audio_decode_envelope() {
    let err = Error::AudioDecode(anyhow::anyhow!("probe failed: unsupported codec"));
    let json = err.to_json("77777777-7777-7777-7777-777777777777");
    insta::assert_json_snapshot!("error_audio_decode", json);
}

#[test]
fn snapshot_unsupported_format_envelope() {
    let err = Error::UnsupportedFormat {
        format: "wma".to_string(),
    };
    let json = err.to_json("88888888-8888-8888-8888-888888888888");
    insta::assert_json_snapshot!("error_unsupported_format", json);
}
