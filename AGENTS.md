[English version](AGENTS.md) | [Versão em Português Brasileiro](AGENTS.pt-BR.md)

# AGENTS — Agent Integration Guide

This document is written for AI agents and orchestration tools that
need to invoke whisper-macos-cli as a black-box transcription service.

## Contract

- One JSON object per file on stdout
- correlation_id is a UUID v7 generated per process invocation
- schema_version reflects the envelope version
- stderr carries tracing logs that can be suppressed with --quiet
- exit codes follow sysexits.h convention

## JSON Output Envelope

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

## Error Envelope

```json
{
  "schema_version": "0.1.0",
  "error": true,
  "code": 66,
  "message": "input not found: /tmp/missing.ogg",
  "category": "input",
  "retryable": false,
  "retry_after_ms": null,
  "hint": "check the file path and try again",
  "docs_url": "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md",
  "correlation_id": "0190a3b4-7c8d-7abc-9def-1234567890ab"
}
```

## Quickstart for Agents

```bash
# 30-second setup
cargo install whisper-macos-cli
whisper-macos-cli models download
whisper-macos-cli transcribe voice.ogg
```

## Token Budget

- Invocation overhead: 200 tokens (envelope parsing)
- Per-file transcription: 50 tokens + transcribed text length
- Error handling: 100 tokens

## Exit Codes

| Code | Meaning                | Retryable |
|------|------------------------|-----------|
| 0    | Success                | n/a       |
| 2    | Usage error            | no        |
| 64   | No input provided      | no        |
| 65   | Invalid audio data      | no        |
| 66   | Input file not found    | no        |
| 69   | Service unavailable    | yes       |
| 70   | Inference error         | no        |
| 74   | I/O error               | no        |
| 78   | Configuration error     | no        |
| 130  | SIGINT (Ctrl+C)         | no        |
| 141  | Broken pipe             | no        |
| 143  | SIGTERM                 | no        |

## Subcommands for Agent Discovery

```bash
# Full command tree as JSON
whisper-macos-cli commands --format json

# Full JSON Schema of output envelope
whisper-macos-cli schema

# Effective configuration
whisper-macos-cli config

# Self-describing skill scaffold
whisper-macos-cli init --target /path/to/agent/workspace
```

## Compatible AI Agents

Claude Code, OpenCode, Codex CLI, Gemini CLI, Cline, Cursor,
Windsurf, Aider, Continue, Cody, Tabnine, Replit Agent. Full list
in [INTEGRATIONS.md](INTEGRATIONS.md).

## Documentation for Deep Integration

See [docs/AGENTS.md](docs/AGENTS.md) for the complete agent-author
guide including agent integration patterns, crate-level contracts,
and CRUD operations.
