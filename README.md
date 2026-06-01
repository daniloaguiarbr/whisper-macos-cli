# whisper-macos-cli


## Overview
- macOS-exclusive CLI for local audio transcription via whisper.cpp
- Metal GPU acceleration on Apple Silicon (M1/M2/M3/M4)
- Stdin/stdout JSON contract for AI agent integration
- Auto-detects transcription language from OS locale
- Maximum quality by default: large-v3 model, BeamSearch beam_size=8


## Prerequisites
- macOS with Apple Silicon
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


## Installation
```bash
cargo install whisper-macos-cli
```


## Quick Start
```bash
# Download the default model (large-v3, ~3GB, one-time)
whisper-macos-cli models download

# Transcribe an audio file
whisper-macos-cli transcribe recording.mp3

# Transcribe from stdin
cat voice.ogg | whisper-macos-cli transcribe

# Multiple files as NDJSON
whisper-macos-cli transcribe *.ogg --ndjson

# Force a specific language
whisper-macos-cli transcribe audio.wav --language en

# Use auto-detection by audio content
whisper-macos-cli transcribe audio.wav --language auto

# Check system prerequisites
whisper-macos-cli doctor
```


## Supported Audio Formats
- MP3
- OGG/Vorbis
- OGG/Opus (WhatsApp voice messages)
- FLAC
- WAV
- AAC (M4A)


## Available Models
| Model | Size | Default | Description |
|---|---|---|---|
| tiny | ~75 MB | | Fastest, lowest accuracy |
| base | ~142 MB | | Fast, basic accuracy |
| small | ~466 MB | | Balanced speed/accuracy |
| medium | ~1.5 GB | | High accuracy |
| large-v3 | ~3 GB | DEFAULT | Maximum accuracy |

```bash
whisper-macos-cli models list       # Show all models and status
whisper-macos-cli models download   # Download default model
whisper-macos-cli models download small  # Download specific model
whisper-macos-cli models path       # Show model file path
whisper-macos-cli models remove tiny     # Remove a model
```


## JSON Output
```json
{
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

- Use `--timestamps` to include timestamped segments
- Use `--ndjson` for one JSON per line per file
- Use `whisper-macos-cli schema` to get full JSON Schema


## Language Detection
- No `--language` flag: detects from macOS system language (e.g. pt-BR → pt)
- `--language pt`: forces Portuguese
- `--language auto`: whisper.cpp auto-detects from audio content


## Exit Codes
- 0: success
- 64: no input
- 65: invalid audio
- 66: file not found
- 69: download failed
- 70: inference error
- 74: I/O error
- 78: config error
- 130: Ctrl+C
- 141: broken pipe


## For AI Agents
- See `AGENTS.md` for integration guide
- JSON contract on stdout, logs on stderr
- `--quiet` suppresses all stderr output
- `whisper-macos-cli schema` emits JSON Schema


## License
- MIT (see LICENSE)
- Third-party notices in NOTICE
