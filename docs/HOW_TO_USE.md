[English version](docs/HOW_TO_USE.md) | [Versão em Português Brasileiro](docs/HOW_TO_USE.pt-BR.md)

> Master the CLI in 60 seconds, from zero to production-grade agent pipeline.

# How To Use

## The Pain

Most transcription tools leak your audio to a third party.
whisper-macos-cli runs entirely on your machine and exposes a
predictable JSON contract. This guide gets you from install to a
production agent integration in under 60 seconds.

## Why

- Whisper Python wrappers add 200 MB and 5 seconds of cold start
- Cloud APIs lock your data behind ToS you cannot audit
- Most CLIs treat stdout as a dumping ground; we treat it as a contract

## Economy

- One 200-line installer script replaces 20 minutes of model setup
- One `whisper-macos-cli transcribe` call replaces 50 lines of Python
- One `--ndjson` flag replaces a custom streaming parser

## Sovereignty

- Your audio never leaves the device
- Your transcriptions never leave the device
- No telemetry, no analytics, no phone-home
- The model is verified by SHA256 before first use

## Prerequisites

- macOS 13 or later
- Apple Silicon (M1, M2, M3, M4)
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust 1.88 or later

## First Command in 60 Seconds

```bash
cargo install whisper-macos-cli
whisper-macos-cli models download
whisper-macos-cli transcribe ~/Desktop/voice-memo.ogg
```

The output is a single JSON object on stdout. The model is cached for
all future invocations.

## Core Commands

### Transcribe a single file

```bash
whisper-macos-cli transcribe audio.ogg
```

### Transcribe from stdin

```bash
cat recording.mp3 | whisper-macos-cli transcribe
```

### Batch with NDJSON

```bash
whisper-macos-cli transcribe *.ogg --ndjson --concurrency 4
```

Each file emits one JSON object per line. A final summary line reports
totals.

### Force a language

```bash
whisper-macos-cli transcribe --language pt audio.wav
```

### Use a smaller model for speed

```bash
whisper-macos-cli models download base
whisper-macos-cli transcribe --model base large-file.wav
```

### Get the JSON Schema for downstream validation

```bash
whisper-macos-cli schema > schema.json
whisper-macos-cli transcribe audio.ogg | jaq -r .text
```

## Advanced Patterns

### Pipe from HTTP

```bash
xh -d https://example.com/audio.ogg | whisper-macos-cli transcribe --quiet
```

### Extract only the text via jaq

```bash
whisper-macos-cli transcribe audio.ogg --quiet | jaq -r '.text'
```

### Dry run for CI validation

```bash
whisper-macos-cli transcribe --dry-run --language pt audio.ogg
```

### Force JSON output in agent mode

```bash
CI=true whisper-macos-cli transcribe --no-input --quiet audio.ogg
```

### Air-gapped transcription

```bash
# Pre-download on a connected machine
whisper-macos-cli models download large-v3

# Copy to the air-gapped machine
scp -r ~/Library/Application\ Support/whisper-macos-cli/ user@airgapped:

# Run with --offline to skip network checks
whisper-macos-cli --offline transcribe --model large-v3 audio.ogg
```

## Configuration

Read the effective configuration at any time:

```bash
whisper-macos-cli config
```

Override defaults via environment variables:

```bash
export WHISPER_MODEL=base
export WHISPER_LANGUAGE=en
export RUST_LOG=info
whisper-macos-cli transcribe audio.ogg
```

## Reference of Subcommands Not Covered Above

| Subcommand  | Purpose                                |
|-------------|----------------------------------------|
| models list | Show installed and available models    |
| models path | Print the model file path             |
| models remove | Delete a downloaded model           |
| doctor      | Diagnose environment and dependencies  |
| commands    | Print full command tree as JSON       |
| init        | Generate skill scaffold                |
| licenses    | Print third-party license attribution  |
| resume      | Resume a previous batch                |

## Integration With AI Agents

See [docs/AGENTS.md](docs/AGENTS.md) for the complete agent-author
guide including compatibility matrix, contract details, and
CRUD-style operations.
