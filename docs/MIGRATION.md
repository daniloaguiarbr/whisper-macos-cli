[English version](MIGRATION.md) | [Versão em Português Brasileiro](MIGRATION.pt-BR.md)

# Migration Guide

## 0.1.0 / 0.1.1 → 0.1.2

### What Changes

Version 0.1.2 introduces video container support and fixes the
OGG/Opus decode bug for WhatsApp voice messages. The `schema_version`
field of every output envelope remains `0.1.0` for backward
compatibility; the schema `$id` URLs reflect the latest schema
surface.

### New Error Variants

Three new variants were added to the `Error` enum. Existing agents
that handle errors via the `code` field continue to work without
changes.

| Variant                | Exit code | Category | Retryable |
|------------------------|-----------|----------|-----------|
| `VideoExtractionFailed`| 65        | data     | no        |
| `FfmpegNotFound`       | 69        | service  | no        |
| `UnsupportedVideoFormat`| 65       | data     | no        |

### New CLI Flags

| Flag                       | Env var                       | Default   | Since |
|----------------------------|-------------------------------|-----------|-------|
| `--ffmpeg-binary <PATH>`   | `WHISPER_FFMPEG_BINARY`       | `ffmpeg`  | 0.1.2 |
| `--no-ffmpeg-fallback`     | `WHISPER_NO_FFMPEG_FALLBACK`  | `false`   | 0.1.2 |

The `--no-ffmpeg-fallback` flag opts out of the transparent
OGG/Opus fallback to ffmpeg. Use it for reproducing native decoder
bugs.

### New Module: `src/video/`

The new `video` module contains the magic bytes detection and
`FfmpegRunner` trait. Public Rust API consumers do not need to
import anything from this module; the CLI handles the routing
transparently.

### New File: `src/audio/decode.rs` signature change

The internal function `decode_file` now takes an optional
`FfmpegRunner` and an `auto_fallback` flag. The public Rust API
is unchanged; the CLI uses the default `RealFfmpeg` runner.

### Step-by-Step Migration

1. Update dependency: `cargo update -p whisper-macos-cli`
2. Verify ffmpeg is installed: `ffmpeg -version`
3. If ffmpeg is not installed, install it: `brew install ffmpeg`
4. Existing pipelines continue to work without changes
5. Optional: pass `--no-ffmpeg-fallback` to disable fallback

### Breaking Changes

None. v0.1.2 is fully backward-compatible with v0.1.0 and v0.1.1.

## 0.1.0 → 0.1.1

### What Changes

Version 0.1.1 only changed the license from MIT-only to dual
MIT OR Apache-2.0. No code or contract changes.

## 0.0.x → 0.1.0

### What Changes

This is the first public release. There is no prior version to
migrate from.

## JSON Schema Changes

The envelope schema is stable within a MAJOR version. Breaking
schema changes require a MAJOR version bump and a `schema_version`
update.

| Field                | Type    | Required | Since |
|----------------------|---------|----------|-------|
| schema_version       | string  | yes      | 0.1.0 |
| correlation_id       | string  | yes      | 0.1.0 |
| file                 | string  | yes      | 0.1.0 |
| language             | string  | yes      | 0.1.0 |
| language_source      | string  | yes      | 0.1.0 |
| model                | string  | yes      | 0.1.0 |
| duration_seconds     | number  | yes      | 0.1.0 |
| text                 | string  | yes      | 0.1.0 |
| segments             | array   | no       | 0.1.0 |
| vad_chunks           | integer | yes      | 0.1.0 |
| processing_time_ms   | integer | yes      | 0.1.0 |

New `transcribe-input` fields added in 0.1.2:

| Field                  | Type    | Required | Since |
|------------------------|---------|----------|-------|
| ffmpeg_binary          | string  | no       | 0.1.2 |
| no_ffmpeg_fallback     | boolean | no       | 0.1.2 |

## Compatibility Notes

The CLI does NOT maintain backward compatibility for flags marked
as internal. Public flags follow SemVer.

## Rollback

To roll back to a previous version:

```bash
cargo install whisper-macos-cli --version 0.1.0 --force
```

## See Also

- [CHANGELOG.md](../CHANGELOG.md) — Full release history
- [docs/TESTING.md](TESTING.md) — Test execution guide
- [CONTRIBUTING.md](../CONTRIBUTING.md) — How to contribute
