---
name: whisper-macos-cli
version: 0.1.2
description: Transcribe audio and video files to text via whisper.cpp with Metal GPU on macOS Apple Silicon. Use when transcribing audio, processing WhatsApp voice messages, transcribing video files, converting speech to text, batch-transcribing files, building AI agent transcription pipelines, or whenever local transcription is required without cloud services.
invariants:
  - stdout is always valid JSON or NDJSON
  - stderr is always human-readable logs
  - exit codes follow sysexits.h convention
  - large-v3 model is the default
  - OGG/Opus (WhatsApp voice messages) is supported natively with automatic ffmpeg fallback
  - video is extracted to WAV via ffmpeg subprocess before transcription
triggers:
  - transcribe audio
  - speech to text
  - audio transcription
  - video transcription
  - whisper.cpp
  - voice message transcription
  - whatsapp audio
  - local transcription
  - batch transcription
---

# whisper-macos-cli

## Capability

Local audio and video transcription via whisper.cpp with Metal GPU
acceleration on macOS Apple Silicon. Accepts audio (MP3, OGG/Vorbis,
OGG/Opus/WhatsApp, FLAC, WAV, AAC) and video (MP4, MOV, M4V, MKV,
WebM, AVI). Video is extracted to WAV via ffmpeg subprocess before
transcription. Emits JSON on stdout with transcription text.

### Video Requirements

- ffmpeg 4.0+ must be available on PATH (or via `--ffmpeg-binary`)
- Install with `brew install ffmpeg` on macOS

## Installation

### REQUIRED

- macOS 13 or later
- Apple Silicon (M1, M2, M3, M4)
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust 1.88 or later: `rustup install stable`

### Correct Pattern

```bash
cargo install whisper-macos-cli
```

## Core Commands

### REQUIRED

- One JSON object per file on stdout
- correlation_id is a UUID v7 generated per process invocation
- schema_version reflects the envelope version
- stderr carries tracing logs that can be suppressed with --quiet
- Exit codes follow sysexits.h convention

### Correct Pattern

```bash
# Single audio file
whisper-macos-cli transcribe voice.ogg

# Video (automatic audio extraction via ffmpeg)
whisper-macos-cli transcribe video.mp4

# Mixed batch with NDJSON
whisper-macos-cli transcribe *.ogg *.mp4 --ndjson --concurrency 4

# From stdin (audio only)
cat audio.mp3 | whisper-macos-cli transcribe

# Custom ffmpeg binary
whisper-macos-cli transcribe --ffmpeg-binary /opt/local/bin/ffmpeg video.mov
```

## JSON Contract

### REQUIRED

Every output on stdout MUST be a valid JSON object with at minimum:
- `schema_version` — string
- `correlation_id` — string (UUID v7)

### Transcription Result

```json
{
  "schema_version": "0.1.0",
  "correlation_id": "0190a3b4-7c8d-7abc-9def-1234567890ab",
  "file": "voice.ogg",
  "language": "pt",
  "language_source": "os_locale",
  "model": "large-v3",
  "duration_seconds": 45.2,
  "text": "Full transcription text here",
  "vad_chunks": 3,
  "processing_time_ms": 8432
}
```

### Error Envelope

```json
{
  "schema_version": "0.1.0",
  "error": true,
  "code": 66,
  "message": "input not found",
  "category": "input",
  "retryable": false,
  "retry_after_ms": null,
  "hint": "check the file path and try again",
  "docs_url": "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md",
  "correlation_id": "0190a3b4-7c8d-7abc-9def-1234567890ab"
}
```

## Exit Codes

| Code | Meaning                                  | Retryable |
|------|------------------------------------------|-----------|
| 0    | Success                                  | n/a       |
| 2    | Usage error                              | no        |
| 64   | No input provided                        | no        |
| 65   | Invalid audio/video data                 | no        |
| 66   | Input file not found                     | no        |
| 69   | Service unavailable (ffmpeg missing)     | yes       |
| 70   | Inference error                          | no        |
| 74   | I/O error                                | no        |
| 78   | Configuration error                      | no        |
| 130  | SIGINT (Ctrl+C)                          | no        |
| 141  | Broken pipe                              | no        |
| 143  | SIGTERM                                  | no        |

## Video and OGG/Opus Auto-Fallback

Since v0.1.2, video files (MP4, MOV, MKV, WebM, AVI) are supported
automatically: audio is extracted via ffmpeg subprocess to a temp
WAV then transcribed. Requires ffmpeg 4.0+ on PATH.

WhatsApp OGG/Opus audio that fails the native symphonia decoder
(upstream codec status "In work") is automatically routed via ffmpeg
as a transparent fallback. No explicit flag required.

Use `--no-ffmpeg-fallback` to disable the fallback (useful for
reproducing native decoder bugs).

## FORBIDDEN

- Never write non-JSON to stdout in transcription mode
- Never use stdout for logs (use stderr)
- Never invoke with `--quiet` when debugging
- Never parse stdout as text (always parse as JSON)
- Never assume a specific exit code without checking
- Never retry a non-retryable error
- Never retry without honoring `retry_after_ms`

## Self-Describing

### REQUIRED

Run `whisper-macos-cli schema` to get the full JSON Schema envelope
including `agentNotes`, `invariants`, `sideEffects`, `idempotent`,
`checkpointable`, and `tokenBudget`.

### Correct Pattern

```bash
# Discover the full command tree
whisper-macos-cli commands --format json

# Emit JSON Schema
whisper-macos-cli schema

# Get effective configuration
whisper-macos-cli config
```

## Model Management

### REQUIRED

The first invocation downloads a model from Hugging Face. The download
is HTTPS-only with User-Agent identification and SHA256 integrity
verification.

### Correct Pattern

```bash
# Download the default model (large-v3, ~3GB)
whisper-macos-cli models download

# Download a smaller model
whisper-macos-cli models download base

# List available models
whisper-macos-cli models list
```

## Composition with Unix Tools

### Correct Pattern

```bash
# Extract text only
whisper-macos-cli transcribe audio.ogg | jaq -r '.text'

# Stream from HTTP
xh -d https://example.com/audio.ogg | whisper-macos-cli transcribe

# Batch via fd
fd -e ogg . /path/to/audios/ \
  | xargs whisper-macos-cli transcribe --ndjson --concurrency 4
```

## Retry Strategy

### REQUIRED

- Honor `retry_after_ms` for retryable errors
- Only retry on exit code 69 (Service unavailable)
- Maximum 3 retry attempts
- Exponential backoff with jitter
- Cancellation via SIGINT or SIGTERM must trigger graceful shutdown

### FORBIDDEN

- Never retry on a non-retryable error
- Never retry without exponential backoff
- Never ignore the `retryable` flag
- Never retry on exit code 78 (configuration error)

## Environment Variables

- `WHISPER_MODEL` — override default model
- `WHISPER_LANGUAGE` — override default language
- `NO_COLOR` — disable colored output
- `CI` — disable interactive prompts when set to 1/true/yes
- `RUST_LOG` — override tracing log level filter
- `SOURCE_DATE_EPOCH` — Unix timestamp for reproducible builds
- `NO_INPUT` — override --no-input flag
- `QUIET` — override --quiet flag
