> Transcribe any audio locally on Apple Silicon in seconds, not minutes.

# whisper-macos-cli

[![Crates.io](https://img.shields.io/crates/v/whisper-macos-cli.svg)](https://crates.io/crates/whisper-macos-cli)
[![Documentation](https://docs.rs/whisper-macos-cli/badge.svg)](https://docs.rs/whisper-macos-cli)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT_OR_Apache--2.0-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/daniloaguiarbr/whisper-macos-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/daniloaguiarbr/whisper-macos-cli/actions)
[![codecov](https://codecov.io/gh/daniloaguiarbr/whisper-macos-cli/branch/main/graph/badge.svg)](https://codecov.io/gh/daniloaguiarbr/whisper-macos-cli)
[![Audit](https://img.shields.io/badge/audit-cargo%20audit-blue)](https://github.com/rustsec/rustsec)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue)](https://github.com/rust-lang/rust/releases)

[Versão em Português Brasileiro](README.pt-BR.md)

## What is it

- Local audio transcription CLI for macOS Apple Silicon
- Powered by whisper.cpp with Metal GPU acceleration
- Strict stdin/stdout JSON contract for AI agents
- Zero telemetry, zero cloud calls, zero setup beyond `cargo install`

## Why

- Audio transcription as a service locks your data with a third party
- Whisper models in Python are 10x slower and 5x heavier than whisper.cpp
- Most CLIs treat stdout as a dumping ground; we treat it as a contract

## Superpowers

- Discoverable: `whisper-macos-cli commands` emits the full command tree
- Self-describing: `whisper-macos-cli schema` returns the full JSON Schema
- Traceable: every output carries a UUID v7 `correlation_id`
- Versioned: every output carries a `schema_version` for safe evolution
- Resilient: SIGINT and SIGTERM trigger clean shutdown; double Ctrl+C forces exit
- Safe: model downloads are SHA256-verified and TLS-enforced
- Composable: behaves like any other Unix tool — pipes, NDJSON, jaq, xargs

## Quick Start

```bash
cargo install whisper-macos-cli
whisper-macos-cli models download
whisper-macos-cli transcribe voice.ogg
```

The first transcription is slower because the model loads into unified
memory. Subsequent transcriptions reuse the cached context.

## Installation

### Prerequisites

- macOS 13 or later
- Apple Silicon (M1, M2, M3, M4)
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust 1.88 or later: `rustup install stable`

### From crates.io

```bash
cargo install whisper-macos-cli
```

### From source

```bash
git clone https://github.com/daniloaguiarbr/whisper-macos-cli
cd whisper-macos-cli
cargo build --release
./target/release/whisper-macos-cli --version
```

### Pre-built binaries

Download the appropriate binary for your architecture from the
[GitHub Releases](https://github.com/daniloaguiarbr/whisper-macos-cli/releases)
page. Verify the SHA256 hash against `SHA256SUMS` before installing.

## Usage

```bash
# Single file
whisper-macos-cli transcribe recording.ogg

# From stdin
cat audio.mp3 | whisper-macos-cli transcribe

# Batch as NDJSON
whisper-macos-cli transcribe *.ogg --ndjson --concurrency 4

# Force a language
whisper-macos-cli transcribe --language pt audio.wav

# Use a smaller model for speed
whisper-macos-cli transcribe --model small audio.wav

# Get JSON Schema for downstream validation
whisper-macos-cli schema > schema.json
whisper-macos-cli transcribe audio.ogg | jsonschema -i schema.json
```

## Commands

| Subcommand  | Purpose                                |
|-------------|----------------------------------------|
| transcribe  | Transcribe one or more audio files     |
| models      | Download, list, locate, or remove models|
| doctor      | Diagnose environment and dependencies  |
| schema      | Emit the full JSON Schema envelope      |
| config      | Emit current effective configuration   |
| commands    | Emit the full command tree as JSON     |
| init        | Generate SKILL.md and AGENTS.md scaffold|
| licenses    | Print third-party license attribution  |
| completions | Generate shell completions             |
| resume      | Resume a previous batch (v0.1: no-op)  |

Run `whisper-macos-cli commands --format json` to see the full tree.

## Environment Variables

| Variable           | Effect                                        |
|--------------------|-----------------------------------------------|
| WHISPER_MODEL      | Override default model                        |
| WHISPER_LANGUAGE   | Override default language                     |
| NO_COLOR           | Disable colored output                        |
| CI                 | Disable interactive prompts (1, true, yes)    |
| RUST_LOG           | Override tracing log level filter             |
| SOURCE_DATE_EPOCH  | Unix timestamp for reproducible builds       |
| NO_INPUT           | Override --no-input flag                      |
| QUIET              | Override --quiet flag                         |

## Integration Patterns

```bash
# Pipe to jaq for selective extraction
whisper-macos-cli transcribe audio.ogg | jaq -r '.text'

# Batch via fd and xargs
fd -e ogg . /path/to/audios/ \
  | xargs whisper-macos-cli transcribe --ndjson --concurrency 4

# Stream from HTTP
xh -d https://example.com/audio.ogg | whisper-macos-cli transcribe

# Validate against schema in CI
whisper-macos-cli transcribe audio.ogg \
  | jaq -e "has(\"correlation_id\") and has(\"schema_version\")"
```

## Performance

- First transcription (cold start, large-v3): 2-5 seconds warmup
- Subsequent transcriptions: roughly real-time on M2 Pro
- Memory: large-v3 requires ~3 GB of unified memory during inference
- Concurrency: scales linearly up to `--concurrency 8` on M1 Pro

## Memory Requirements

| Model    | Peak Memory |
|----------|-------------|
| tiny     | ~300 MB     |
| base     | ~500 MB     |
| small    | ~1 GB       |
| medium   | ~3 GB       |
| large-v3 | ~3.5 GB     |

Whisper.cpp unloads the model when the process exits.

## Troubleshooting FAQ

See [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for the complete
guide, including:

- exit code 64 (no input)
- exit code 65 (invalid audio)
- exit code 66 (file not found)
- exit code 69 (download failed)
- exit code 70 (inference failed)
- exit code 74 (I/O error)
- exit code 78 (model not found)

## Documentation

- [AGENTS.md](AGENTS.md) — Agent integration guide
- [CHANGELOG.md](CHANGELOG.md) — Release history
- [CONTRIBUTING.md](CONTRIBUTING.md) — How to contribute
- [SECURITY.md](SECURITY.md) — Report vulnerabilities
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — Community standards
- [PRIVACY.md](PRIVACY.md) — Data handling policy
- [INTEGRATIONS.md](INTEGRATIONS.md) — Supported agents and platforms
- [llms.txt](llms.txt) — LLM-friendly summary
- [llms-full.txt](llms-full.txt) — LLM-friendly full reference
- [docs/HOW_TO_USE.md](docs/HOW_TO_USE.md) — Advanced recipes
- [docs/AGENTS.md](docs/AGENTS.md) — Author guide for agent integrators
- [docs/COOKBOOK.md](docs/COOKBOOK.md) — Twenty worked examples
- [docs/CROSS_PLATFORM.md](docs/CROSS_PLATFORM.md) — Platform matrix
- [docs/MIGRATION.md](docs/MIGRATION.md) — Version migration
- [docs/TESTING.md](docs/TESTING.md) — Test execution guide
- [docs/schemas/](docs/schemas/README.md) — Machine-readable schemas
- [skill/](skill/) — Agent skill descriptors

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the workflow. Every pull
request must pass the 8-item checklist before merge.

## Security

Report vulnerabilities via GitHub Security Advisories at
https://github.com/daniloaguiarbr/whisper-macos-cli/security/advisories/new
— not as public issues. SLA is 72 hours for initial triage.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for the complete release history. The
current development version is documented under `## [Unreleased]`.

## License

Dual-licensed under either of:

- Apache License, Version 2.0 — see [LICENSE-APACHE](LICENSE-APACHE)
- MIT License — see [LICENSE-MIT](LICENSE-MIT)

at your option. Third-party notices in [NOTICE](NOTICE) and via
`whisper-macos-cli licenses`.
