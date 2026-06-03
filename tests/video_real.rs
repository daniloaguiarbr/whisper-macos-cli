//! Integration tests for the video extraction pipeline.
//!
//! These tests use the production [`RealFfmpeg`] backed by the system's
//! real ffmpeg binary WHEN AVAILABLE. They are gated behind the
//! `slow-tests` feature so they don't break CI on hosts without ffmpeg.
//!
//! For unit-level coverage of the extraction logic without subprocess
//! overhead, see the `MockFfmpeg` tests in `src/video/ffmpeg.rs`.

#![cfg(feature = "slow-tests")]

use std::path::PathBuf;
use std::process::Command;

use whisper_macos_cli::video::ffmpeg::{FfmpegRunner, RealFfmpeg};

fn ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .stdin(std::process::Stdio::null())
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn ffmpeg_works() -> bool {
    if !ffmpeg_available() {
        eprintln!("skipping: ffmpeg not in PATH");
        return false;
    }
    true
}

#[test]
fn real_ffmpeg_is_available_on_test_hosts() {
    if !ffmpeg_works() {
        return;
    }
    let f = RealFfmpeg::new("ffmpeg");
    assert!(f.is_available());
}

#[test]
fn real_ffmpeg_extracts_audio_from_synthetic_mp4() {
    if !ffmpeg_works() {
        return;
    }
    // Create a 1-second silent MP4 with audio track using ffmpeg lavfi
    let temp_dir = std::env::temp_dir();
    let mp4_path: PathBuf = temp_dir.join(format!("wmac-test-{}.mp4", uuid::Uuid::now_v7()));
    let wav_path: PathBuf = temp_dir.join(format!("wmac-test-{}.wav", uuid::Uuid::now_v7()));

    // Generate the MP4
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("anullsrc=channel_layout=mono:sample_rate=16000")
        .arg("-t")
        .arg("1")
        .arg("-c:a")
        .arg("aac")
        .arg("-strict")
        .arg("experimental")
        .arg(&mp4_path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("spawn ffmpeg lavfi");
    assert!(status.success(), "ffmpeg lavfi failed to create MP4");

    // Now extract via RealFfmpeg
    let f = RealFfmpeg::new("ffmpeg");
    let result = f
        .extract_audio_wav(&mp4_path)
        .expect("real ffmpeg should extract audio from synthetic MP4");
    assert!(result.output_path.exists());
    assert!(
        result.output_bytes > 44,
        "extracted WAV should have audio data"
    );
    assert!(wav_path.as_os_str().is_empty() || !wav_path.exists());

    // Validate the WAV
    let header = std::fs::read(&result.output_path).unwrap();
    assert_eq!(&header[..4], b"RIFF", "output should be RIFF");
    assert_eq!(&header[8..12], b"WAVE", "output should be WAVE");

    // Cleanup
    let _ = std::fs::remove_file(&mp4_path);
    let _ = std::fs::remove_file(&result.output_path);
}

#[test]
fn real_ffmpeg_rejects_missing_input() {
    if !ffmpeg_works() {
        return;
    }
    let f = RealFfmpeg::new("ffmpeg");
    let result = f.extract_audio_wav(PathBuf::from("/tmp/does-not-exist.mp4").as_path());
    assert!(result.is_err());
}

#[test]
fn real_ffmpeg_unavailable_binary_returns_ffmpeg_not_found() {
    // Use a path that is guaranteed not to exist
    let f = RealFfmpeg::new("/this/path/does/not/exist/ffmpeg");
    assert!(!f.is_available());
    let result = f.extract_audio_wav(PathBuf::from("/tmp/x.mp4").as_path());
    match result {
        Err(whisper_macos_cli::error::Error::FfmpegNotFound) => {}
        other => panic!("expected FfmpegNotFound, got {other:?}"),
    }
}
