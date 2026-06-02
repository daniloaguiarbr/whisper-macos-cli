use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("whisper-macos-cli").unwrap()
}

#[test]
fn clap_debug_assert() {
    use clap::CommandFactory;
    whisper_macos_cli::cli::Cli::command().debug_assert();
}

#[test]
fn help_flag_succeeds() {
    cmd().arg("--help").assert().success();
}

#[test]
fn version_shows_semver_and_target() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("whisper-macos-cli"))
        .stdout(predicates::str::contains("0.1.0"));
}

#[test]
fn schema_subcommand_outputs_valid_json() {
    let output = cmd().arg("schema").assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["result_schema"]["title"], "TranscriptionResult");
    assert!(parsed["agentNotes"].is_string());
    assert!(parsed["invariants"].is_array());
}

#[test]
fn print_schema_flag_outputs_json() {
    let output = cmd().arg("--print-schema").assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let _: serde_json::Value = serde_json::from_str(&stdout).unwrap();
}

#[test]
fn transcribe_nonexistent_file_exits_66() {
    cmd()
        .args(["transcribe", "/tmp/nonexistent_audio_file_xyz.ogg"])
        .assert()
        .code(66)
        .stdout(
            predicates::str::contains(r#""error":true"#)
                .or(predicates::str::contains(r#""error": true"#)),
        );
}

#[test]
fn transcribe_no_input_tty_exits_64() {
    cmd().arg("transcribe").assert().code(64);
}

#[test]
fn models_list_succeeds() {
    cmd().args(["models", "list"]).assert().success();
}

#[test]
fn models_path_succeeds() {
    cmd().args(["models", "path"]).assert().success();
}

#[test]
fn doctor_succeeds() {
    cmd().arg("doctor").assert().success();
}

#[test]
fn config_subcommand_works() {
    let output = cmd().arg("config").assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["config"].is_object());
    assert!(parsed["correlation_id"].is_string());
}

#[test]
fn commands_subcommand_outputs_tree() {
    let output = cmd()
        .args(["commands", "--format", "json"])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["subcommands"].is_array());
}

#[test]
fn licenses_subcommand_works() {
    let output = cmd().arg("licenses").assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["license"], "MIT");
}

#[test]
fn init_subcommand_creates_skill_files() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().to_path_buf();
    let output = cmd()
        .args(["init", "--target", target.to_str().unwrap()])
        .assert()
        .success();
    assert!(target.join("SKILL.md").exists());
    assert!(target.join("AGENTS.md").exists());
    let _ = output;
}

#[test]
fn dry_run_emits_envelope_without_transcribing() {
    let output = cmd()
        .args(["transcribe", "--dry-run", "/tmp/some_file.ogg"])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["dry_run"], true);
    assert!(parsed["would_transcribe"].is_object());
}

#[test]
fn schema_envelope_contains_required_agent_fields() {
    let output = cmd().arg("schema").assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["agentNotes"].is_string());
    assert!(parsed["invariants"].is_array());
    assert!(parsed["sideEffects"].is_array());
    assert!(parsed["idempotent"].is_boolean());
    assert!(parsed["checkpointable"].is_boolean());
    assert!(parsed["tokenBudget"].is_object());
}

#[test]
fn transcribe_invalid_model_exits_2() {
    cmd()
        .args([
            "transcribe",
            "--model",
            "nonexistent-model",
            "/tmp/test.ogg",
        ])
        .assert()
        .code(2);
}

#[test]
fn transcribe_beam_size_zero_rejected() {
    cmd()
        .args(["transcribe", "--beam-size", "0", "/tmp/test.ogg"])
        .assert()
        .code(2)
        .stderr(predicates::str::contains(
            "beam size must be between 1 and 16",
        ));
}

#[test]
fn transcribe_vad_threshold_out_of_range_rejected() {
    cmd()
        .args(["transcribe", "--vad-threshold", "1.5", "/tmp/test.ogg"])
        .assert()
        .code(2)
        .stderr(predicates::str::contains(
            "VAD threshold must be between 0.0 and 1.0",
        ));
}

#[test]
fn transcribe_concurrency_zero_rejected() {
    cmd()
        .args(["transcribe", "--concurrency", "0", "/tmp/test.ogg"])
        .assert()
        .code(2)
        .stderr(predicates::str::contains(
            "concurrency must be between 1 and 32",
        ));
}

#[test]
fn no_command_shows_help() {
    cmd()
        .assert()
        .code(2)
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn completions_bash_succeeds() {
    cmd()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("whisper-macos-cli"));
}

#[test]
fn error_json_includes_hint() {
    let output = cmd()
        .args(["transcribe", "/tmp/nonexistent_audio_file_xyz.ogg"])
        .assert()
        .code(66);
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["error"], true);
    assert!(parsed["hint"].is_string());
    assert!(parsed["docs_url"].is_string());
    assert!(parsed["correlation_id"].is_string());
    assert!(parsed["schema_version"].is_string());
}

#[test]
fn models_list_outputs_json() {
    let output = cmd().args(["models", "list"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["models"].is_array());
    assert!(parsed["models"].as_array().unwrap().len() >= 5);
    assert!(parsed["correlation_id"].is_string());
}

#[test]
fn models_path_outputs_json() {
    let output = cmd().args(["models", "path"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["model"].is_string());
    assert!(parsed["path"].is_string());
}

#[test]
fn doctor_outputs_json() {
    let output = cmd().arg("doctor").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["checks"].is_array());
    assert!(parsed["all_ok"].is_boolean());
}

#[test]
fn models_remove_dry_run() {
    let output = cmd()
        .args(["models", "remove", "tiny", "--dry-run"])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["action"], "would_remove");
    assert_eq!(parsed["model"], "tiny");
}

#[test]
fn help_includes_exit_status_section() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("EXIT STATUS:"));
}

#[test]
fn version_includes_build_date() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::is_match(r"\d{4}-\d{2}-\d{2}").unwrap());
}

#[test]
fn no_input_flag_accepted() {
    cmd().args(["--no-input", "transcribe"]).assert().code(64);
}

#[test]
fn output_format_ndjson_accepted() {
    let output = cmd()
        .args([
            "transcribe",
            "--output-format",
            "ndjson",
            "/tmp/nonexistent_audio_file_xyz.ogg",
        ])
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("\"error\":true") || stdout.contains("\"error\": true"));
}

#[test]
fn transcribe_real_whatsapp_audio() {
    let test_audio = "/tmp/test_whatsapp_audio.ogg";
    if !std::path::Path::new(test_audio).exists() {
        eprintln!("skipping: test audio not found at {test_audio}");
        return;
    }

    let output = cmd()
        .args(["transcribe", test_audio, "--quiet"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    assert_eq!(parsed["file"], "test_whatsapp_audio.ogg");
    assert!(parsed["language"].is_string());
    assert!(parsed["text"].as_str().unwrap().len() > 5);
    assert!(parsed["duration_seconds"].as_f64().unwrap() > 0.0);
    assert!(parsed["processing_time_ms"].as_u64().unwrap() > 0);
}
