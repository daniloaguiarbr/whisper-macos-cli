//! Video container detection and audio extraction.
//!
//! This module detects video containers (MP4, MOV, MKV, AVI, WebM, M4V) by
//! magic bytes and extension, then extracts the audio track to a temporary
//! WAV file using ffmpeg subprocess. The extracted WAV is then consumed
//! by the existing audio decode pipeline.
//!
//! # Safety
//!
//! - ffmpeg subprocess runs with `env_clear()` to prevent env var leaks
//! - child handle is wrapped in `SafeChild` that kills on drop (panic-safe)
//! - bounded timeout via `wait_timeout` polling prevents infinite hangs
//! - temp output file is removed via `Drop` guard even on panic
//! - user-supplied `--ffmpeg-binary` is invoked via `Command::args` (no shell)
//! - magic bytes are validated BEFORE ffmpeg invocation (input sanity)

pub mod ffmpeg;

use std::path::Path;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "m4v", "mov", "mkv", "webm", "avi", "m4a"];

/// Return `true` if `path` has a recognized video file extension.
///
/// The check is case-insensitive. Pure extension match is sufficient for
/// common cases but is intentionally paired with [`is_video_magic_bytes`]
/// to defend against renamed files.
#[must_use]
pub fn is_video_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| {
            let lower = ext.to_ascii_lowercase();
            VIDEO_EXTENSIONS.iter().any(|v| *v == lower)
        })
        .unwrap_or(false)
}

/// Return `true` if `header` begins with magic bytes for a known video
/// container.
///
/// - MP4 / MOV / M4V / M4A: `....ftyp` at offset 4 with brand in {`isom`,
///   `mp42`, `M4V `, `M4A `, `qt  `, `3gp5`, `3gp6`}
/// - AVI: `RIFF....AVI ` at offset 0
/// - MKV / WebM: EBML header `0x1A 0x45 0xDF 0xA3` at offset 0
/// - FLV: `FLV\x01` at offset 0
/// - WMV / WMA: ASF header `0x30 0x26 0xB2 0x75` at offset 0
///
/// `header` may be shorter than 12 bytes; the function returns `false`
/// for headers below the minimum required for any container.
#[must_use]
pub fn is_video_magic_bytes(header: &[u8]) -> bool {
    if header.len() < 4 {
        return false;
    }

    if header.len() >= 12 && &header[4..8] == b"ftyp" {
        let brand = &header[8..12];
        if brand == b"isom"
            || brand == b"mp42"
            || brand == b"M4V "
            || brand == b"M4A "
            || brand == b"qt  "
            || brand == b"3gp5"
            || brand == b"3gp6"
        {
            return true;
        }
    }

    if header.len() >= 12 && &header[..4] == b"RIFF" && &header[8..12] == b"AVI " {
        return true;
    }

    if header.len() >= 4 && header[..4] == [0x1A, 0x45, 0xDF, 0xA3] {
        return true;
    }

    if header.len() >= 4 && header[..4] == *b"FLV\x01" {
        return true;
    }

    if header.len() >= 4 && header[..4] == [0x30, 0x26, 0xB2, 0x75] {
        return true;
    }

    false
}

/// Decide whether `path` should be routed to the video extraction pipeline.
///
/// Combines extension and magic-byte detection. Magic bytes take priority
/// when both are present (defends against renamed audio masquerading as
/// video and vice versa).
#[must_use]
pub fn should_extract_from_video(path: &Path, header: &[u8]) -> bool {
    if is_video_magic_bytes(header) {
        return true;
    }
    is_video_extension(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_video_extension_accepts_mp4() {
        assert!(is_video_extension(Path::new("video.mp4")));
        assert!(is_video_extension(Path::new("VIDEO.MP4")));
    }

    #[test]
    fn is_video_extension_accepts_all_supported() {
        for ext in ["mp4", "m4v", "mov", "mkv", "webm", "avi", "m4a"] {
            assert!(
                is_video_extension(Path::new(&format!("file.{ext}"))),
                "expected {ext} to be recognized"
            );
        }
    }

    #[test]
    fn is_video_extension_rejects_audio() {
        for ext in ["ogg", "mp3", "wav", "flac", "opus", "m4a-rdm"] {
            assert!(
                !is_video_extension(Path::new(&format!("audio.{ext}"))),
                "expected {ext} NOT to be recognized as video"
            );
        }
    }

    #[test]
    fn is_video_extension_rejects_no_extension() {
        assert!(!is_video_extension(Path::new("audio")));
        assert!(!is_video_extension(Path::new("/tmp/somefile")));
    }

    #[test]
    fn is_video_magic_bytes_rejects_short_input() {
        assert!(!is_video_magic_bytes(b""));
        assert!(!is_video_magic_bytes(b"\x00"));
        assert!(!is_video_magic_bytes(b"RI"));
        assert!(!is_video_magic_bytes(b"ftyp"));
    }

    #[test]
    fn is_video_magic_bytes_detects_mp4_isom_brand() {
        let mp4 = b"\x00\x00\x00\x20ftypisom";
        assert!(is_video_magic_bytes(mp4));
    }

    #[test]
    fn is_video_magic_bytes_detects_mp4_mp42_brand() {
        let mp4 = b"\x00\x00\x00\x20ftypmp42";
        assert!(is_video_magic_bytes(mp4));
    }

    #[test]
    fn is_video_magic_bytes_detects_m4v_brand() {
        let m4v = b"\x00\x00\x00\x20ftypM4V ";
        assert!(is_video_magic_bytes(m4v));
    }

    #[test]
    fn is_video_magic_bytes_detects_m4a_brand() {
        let m4a = b"\x00\x00\x00\x20ftypM4A ";
        assert!(is_video_magic_bytes(m4a));
    }

    #[test]
    fn is_video_magic_bytes_detects_qt_brand() {
        let qt = b"\x00\x00\x00\x20ftypqt  ";
        assert!(is_video_magic_bytes(qt));
    }

    #[test]
    fn is_video_magic_bytes_detects_avi() {
        let avi = b"RIFF\x00\x00\x00\x00AVI ";
        assert!(is_video_magic_bytes(avi));
    }

    #[test]
    fn is_video_magic_bytes_rejects_riff_non_avi() {
        let wave = b"RIFF\x00\x00\x00\x00WAVE";
        assert!(!is_video_magic_bytes(wave));
    }

    #[test]
    fn is_video_magic_bytes_detects_ebml_mkv_webm() {
        let mkv = [0x1A, 0x45, 0xDF, 0xA3, 0x93, 0x42, 0x82, 0x88];
        assert!(is_video_magic_bytes(&mkv));
        assert!(is_video_magic_bytes(&mkv[..4]));
    }

    #[test]
    fn is_video_magic_bytes_detects_flv() {
        assert!(is_video_magic_bytes(b"FLV\x01"));
    }

    #[test]
    fn is_video_magic_bytes_detects_wmv_wma_asf() {
        let asf = [0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11];
        assert!(is_video_magic_bytes(&asf));
    }

    #[test]
    fn is_video_magic_bytes_rejects_audio_containers() {
        assert!(!is_video_magic_bytes(b"RIFF\x00\x00\x00\x00WAVE"));
        assert!(!is_video_magic_bytes(b"OggS\x00\x02\x00\x00"));
        assert!(!is_video_magic_bytes(b"ID3\x04\x00\x00\x00"));
        assert!(!is_video_magic_bytes(b"\xFF\xFB\x90\x00"));
        assert!(!is_video_magic_bytes(b"fLaC\x00\x00\x00"));
    }

    #[test]
    fn should_extract_from_video_prioritizes_magic_bytes() {
        let mp4_header = b"\x00\x00\x00\x20ftypisom";
        assert!(should_extract_from_video(Path::new("file.mp4"), mp4_header));
        assert!(should_extract_from_video(
            Path::new("file.unknown"),
            mp4_header
        ));
    }

    #[test]
    fn should_extract_from_video_uses_extension_fallback() {
        assert!(should_extract_from_video(Path::new("video.mov"), &[]));
        assert!(should_extract_from_video(Path::new("video.mkv"), b""));
    }

    #[test]
    fn should_extract_from_video_rejects_pure_audio() {
        assert!(!should_extract_from_video(
            Path::new("voice.ogg"),
            b"OggS\x00\x02"
        ));
        assert!(!should_extract_from_video(Path::new("voice.mp3"), b"ID3"));
    }
}
