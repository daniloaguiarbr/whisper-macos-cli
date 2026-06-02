# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- License changed from MIT-only to dual MIT OR Apache-2.0
- `LICENSE-MIT` and `LICENSE-APACHE` replace the single `LICENSE` file
- `Cargo.toml` license field is now `MIT OR Apache-2.0`

### Added

- SHA256 integrity verification on model downloads
- User-Agent identification in HTTP requests
- Retry with exponential backoff for transient download errors
- Magic byte validation before audio decode
- OGG/Opus pre-skip (3840 samples @ 48kHz) automatic discard
- 24h maximum audio duration limit (DoS protection)
- 2 GB maximum stdin size limit (OOM protection)
- NFC normalization of transcription text
- `correlation_id` (UUID v7) in every JSON output
- `schema_version` field in every JSON output
- `docs_url` field in error envelope
- `retry_after_ms` field in error envelope
- NDJSON summary line at end of batch operations
- SIGINT handler with cleanup (no longer calls `process::exit`)
- SIGTERM handler with graceful shutdown
- Double-tap Ctrl+C forces immediate exit
- New subcommands: `commands`, `init`, `licenses`, `config`, `resume`
- New global flags: `--print-config`, `--print-schema`, `--no-input`
- New transcribe flags: `--dry-run`, `--timeout`, `--retry-count`,
  `--retry-max-elapsed`, `--offline`, `--resume`
- `WHISPER_MODEL` and `WHISPER_LANGUAGE` environment variable support
- `CI=true` environment variable honored
- `doc-url` per error category
- `air-gapped` detection in `doctor` subcommand
- `disk space` check in `doctor` subcommand
- Network connectivity probe in `doctor` subcommand
- `THIRD-PARTY-LICENSES.md` and `THIRD-PARTY-LICENSES` subcommand
- `CONTRIBUTING.md` with 8-item PR checklist
- `SECURITY.md` with 72h SLA and coordinated disclosure policy
- `CODE_OF_CONDUCT.md` (Contributor Covenant v2.1)
- `PRIVACY.md` documenting data handling
- `llms-full.txt` for comprehensive LLM consumption
- `README.pt-BR.md` Brazilian Portuguese translation
- `AGENTS.pt-BR.md` Brazilian Portuguese integration guide
- `docs/HOW_TO_USE.md` (10+ advanced recipes)
- `docs/COOKBOOK.md` (20+ worked examples)
- `docs/INTEGRATIONS.md` (35+ integrations)
- `docs/CROSS_PLATFORM.md` (platform support matrix)
- `docs/TROUBLESHOOTING.md` (error solutions)
- `docs/FAQ.md` (frequently asked questions)
- `deny.toml` for cargo-deny license and advisory checks
- `.cargo/audit.toml` for cargo-audit configuration
- `.cargo/config.toml` for build reproducibility
- `.editorconfig` for cross-editor consistency
- `.gitattributes` for line ending normalization
- `.github/workflows/ci.yml` with matrix, audit, deny, doc, coverage
- `.github/workflows/release.yml` for cross-platform release builds
- `.github/dependabot.yml` for automated dependency updates
- `.github/ISSUE_TEMPLATE/bug.md` for structured bug reports
- `.github/PULL_REQUEST_TEMPLATE.md` with 12-item checklist
- `proptest` and `insta` dev dependencies
- `criterion` benchmark scaffolding
- `wiremock` for HTTP mock testing
- `serial_test` for serial test execution

### Changed

- Model registry now stores `min_size_bytes` for partial-download
  rejection
- `error::Error::to_json` requires `correlation_id` parameter
- All commands now propagate `correlation_id` through the call stack
- `output::write_error` now requires `correlation_id` parameter
- `signal::install_ctrlc_handler` renamed to `install_handlers`
  and adds SIGTERM support
- `eprintln!` replaced with `tracing::info!` in `transcribe.rs` and
  `models.rs`
- Transcription text normalized to Unicode NFC before serialization
- Build requires Rust 1.88 MSRV

### Security

- Added `# Safety` documentation to all `unsafe` blocks
- Added SHA256 verification on model download
- Added User-Agent identification on all HTTP requests
- Added `min_size_bytes` check to reject partial downloads
- Added `cleanup_partial_downloads` to remove temp files on signal
- Added explicit retry classification (5xx and 429 are transient)
- Added 24h DoS protection limit on audio duration
- Added 2 GB OOM protection on stdin size

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

[Unreleased]: https://github.com/daniloaguiarbr/whisper-macos-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/daniloaguiarbr/whisper-macos-cli/releases/tag/v0.1.0
