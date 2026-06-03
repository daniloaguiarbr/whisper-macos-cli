# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Documentation

- Full bilingual documentation reviewed and validated against the
  `rules_rust_documentation_framework`
- Added `THIRD-PARTY-LICENSES.md` for crates.io publication
- Added `MIGRATION.md` covering the 0.1.0 → 0.1.2 transition
- Added video and ffmpeg Q&A to `FAQ.md`
- Added video and ffmpeg recipes to `COOKBOOK.md`
- Added ffmpeg subprocess sections to `SECURITY.md` and
  `PRIVACY.md`
- Added sections for the new error variants
  `VideoExtractionFailed`, `FfmpegNotFound`, `UnsupportedVideoFormat`
  to `TROUBLESHOOTING.md`
- JSON schemas bumped to `$id` v0.1.2 with new `ffmpeg_binary`
  and `no_ffmpeg_fallback` fields in `transcribe-input`
- `Cargo.toml` migrated from `include` to `exclude` (inverted
  allowlist per the framework)
- Contributor Covenant badge added to `CODE_OF_CONDUCT.md`
- All MCP references removed per project policy
- `llms.txt` and `llms-full.txt` updated with `PRIVACY.md` and
  `docs/VIDEO-EXTRACTION.md` links
- `docs/AGENTS.md` hybrid AIDA structure added (Why/Economy/Sovereignty)

## [0.1.2] - 2026-06-02

### Added

- Video container support: MP4, MOV, M4V, MKV, WebM, AVI, M4A
- Auto-extraction of audio track from video via ffmpeg subprocess
- `Error::VideoExtractionFailed` (exit 65) when ffmpeg fails
- `Error::FfmpegNotFound` (exit 69) when ffmpeg binary is missing
- `Error::UnsupportedVideoFormat` (exit 65) when `--no-ffmpeg-fallback` is set
- `--ffmpeg-binary <PATH>` flag (env: `WHISPER_FFMPEG_BINARY`)
- `--no-ffmpeg-fallback` flag (env: `WHISPER_NO_FFMPEG_FALLBACK`) to opt out
- `docs/VIDEO-EXTRACTION.md` (English) and `docs/VIDEO-EXTRACTION.pt-BR.md` (PT-BR)
- ffmpeg subprocess wrapper with `FfmpegRunner` trait, `RealFfmpeg` and `MockFfmpeg`
- 17 unit tests in `src/video/mod.rs` for magic bytes detection
- 23 unit tests in `src/video/ffmpeg.rs` for subprocess hardening
- 12 integration tests in `tests/video_extraction.rs` covering routing logic

### Fixed

- OGG/Opus decode failure for WhatsApp voice messages (symphonia Issue #8):
  transparent fallback to ffmpeg when native decode fails, with full
  error capture and bounded timeout

### Changed

- `decode_file` now takes optional ffmpeg runner and auto-fallback flag
- Default behavior: ffmpeg fallback is enabled but only triggers on
  actual decode failure, not on success
- `Error` enum marked `#[non_exhaustive]` for stable evolution
- `Error::VideoExtractionFailed` field renamed from `source` to `path`
  to avoid conflict with `thiserror` `source` semantics

### Security

- ffmpeg subprocess runs with `env_clear()` plus minimal allowlist
  (`PATH`, `HOME`, `TMPDIR`, `LANG`, `LC_ALL`) to prevent secret leaks
- Child process wrapped in `SafeChild` with kill-on-drop semantics;
  no zombie ffmpeg processes on panic
- Unix: child runs in own process group via `setsid()` so SIGINT to
  parent does not cascade
- Windows: `CREATE_NEW_PROCESS_GROUP` for same isolation
- Temp WAV files cleaned up via `Drop` guard even on panic
- Magic bytes validated BEFORE ffmpeg invocation to refuse renamed
  non-video files
- Bounded timeout (180s default) prevents infinite hangs

## [0.1.1] - 2026-06-02

### Changed

- License changed from MIT-only to dual MIT OR Apache-2.0
- `LICENSE-MIT` and `LICENSE-APACHE` replace the single `LICENSE` file
- `Cargo.toml` license field is now `MIT OR Apache-2.0`

## [0.1.0] - 2026-06-01

### Added

- Initial release
- Audio transcription via whisper.cpp with Metal GPU acceleration
- Support for MP3, WAV, FLAC, AAC, OGG/Vorbis, OGG/Opus (WhatsApp voice messages)
- 5 model sizes: tiny, base, small, medium, large-v3 (default)
- VAD (Voice Activity Detection) via Silero for hallucination prevention
- Automatic language detection from macOS system locale
- `--language auto` mode for whisper.cpp native language detection
- JSON and NDJSON output modes for AI agent integration
- Parallel transcription via `--concurrency`
- `doctor` subcommand for environment diagnostics
- `schema` subcommand for JSON schema introspection
- `completions` subcommand for shell completion generation
- `--print-schema` global flag
- `--color` flag with auto/always/never modes
- Structured error JSON in stdout with category and retryable fields
- Atomic model downloads with progress bar
- BeamSearch decoding with configurable beam size (default 8)
- Hallucination filtering and consecutive repeat collapsing
- AGENTS.md, SKILL.md, llms.txt for LLM agent discovery

[Unreleased]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/daniloaguiarbr/whisper-macos-cli/releases/tag/v0.1.0
