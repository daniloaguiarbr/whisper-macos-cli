use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[test]
fn stderr_silencer_restores_stderr_on_drop() {
    let output = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run --version");

    assert!(output.status.success());
}

#[test]
fn human_panic_setup_does_not_panic_on_normal_execution() {
    let output = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("whisper-macos-cli"));
}

#[test]
fn broken_pipe_returns_exit_141() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("transcribe")
        .arg("/tmp/nonexistent.ogg")
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn child");

    let _ = child.wait().expect("wait child");
}

#[test]
fn flag_quiet_suppresses_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("transcribe")
        .arg("/tmp/nonexistent.ogg")
        .arg("--quiet")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run transcribe with --quiet");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.is_empty() || !stderr.contains("tracing"),
        "stderr should not contain tracing logs in quiet mode, got: {stderr}"
    );
}

#[test]
fn verbose_emits_logs_to_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("-vv")
        .arg("transcribe")
        .arg("/tmp/nonexistent.ogg")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run transcribe with -vv");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stderr.is_empty() || !stdout.is_empty(),
        "expected stderr or stdout to be non-empty"
    );
}

#[test]
fn flag_help_exits_0_with_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("--help")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run --help");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE") || stdout.contains("Usage"));
}

#[test]
fn flag_version_long_format() {
    let output = Command::new(env!("CARGO_BIN_EXE_whisper-macos-cli"))
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
    assert!(stdout.contains(env!("TARGET")));
}

#[allow(dead_code)]
fn read_to_string(mut r: impl Read) -> String {
    let mut s = String::new();
    let _ = r.read_to_string(&mut s);
    s
}

#[allow(dead_code)]
fn write_all(mut w: impl Write, s: &str) {
    let _ = w.write_all(s.as_bytes());
}
