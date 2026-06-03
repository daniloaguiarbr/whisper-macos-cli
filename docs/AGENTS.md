[English version](docs/AGENTS.md) | [Versão em Português Brasileiro](docs/AGENTS.pt-BR.md)

> Ship a transcription skill in one afternoon, not one week. Free, local, predictable.

# AGENTS — Author Guide

This document is written for engineers building AI agents that
integrate whisper-macos-cli as a black-box transcription service.

## Why whisper-macos-cli for Agents

- The CLI is a single Rust binary with zero runtime dependencies
  besides whisper.cpp and the macOS Metal stack
- Subprocess invocation is the only integration surface, with a
  stable JSON envelope on stdout and a sysexits.h exit code
  convention
- Every output carries a `correlation_id` (UUID v7) and a
  `schema_version`, enabling traceable agent workflows and safe
  contract evolution

## Economy

- 200 tokens of invocation overhead (envelope parsing)
- 50 tokens per transcribed file plus the transcribed text length
- 100 tokens of error handling per failed invocation
- No re-embedding, no model hot-reload, no warmup costs across
  invocations when `--concurrency` is held constant

## Sovereignty

- Audio and transcriptions never leave the device
- No telemetry, no analytics, no phone-home
- Model weights are SHA256-verified before first use
- All processing happens on user-controlled macOS Apple Silicon

## Compatible Agents and Orchestrators

- Claude Code (Anthropic)
- OpenCode
- Codex CLI (OpenAI)
- Gemini CLI (Google)
- Cline
- Cursor
- Windsurf
- Aider
- Continue
- Cody (Sourcegraph)
- Tabnine
- Replit Agent

## Agent Integration Details

Every agent integrates by invoking the binary via subprocess and
parsing the JSON envelope on stdout. The contract is stable and
versioned via `schema_version`.

```python
import subprocess
import json

result = subprocess.run(
    ["whisper-macos-cli", "transcribe", "--quiet", audio_path],
    capture_output=True, text=True, check=True
)
output = json.loads(result.stdout)
text = output["text"]
correlation_id = output["correlation_id"]
```

## Crate Integrations

The CLI is published as a single binary on crates.io. The Rust
ecosystem crates that compose the binary are documented in
[docs/COOKBOOK.md](docs/COOKBOOK.md) and at
`whisper-macos-cli licenses`.

## Technical Contract (CRUD-Style)

### Read (transcribe)

- Input: file path or stdin
- Output: JSON envelope on stdout
- Side effects: may write to model cache directory; may invoke
  ffmpeg subprocess for video or OGG/Opus fallback
- Idempotent: yes (same input, same model, same output)
- Checkpointable: no
- Supported formats (v0.1.2+): MP3, WAV, FLAC, AAC, OGG/Vorbis,
  OGG/Opus, MP4, MOV, M4V, MKV, WebM, AVI, FLV, WMV/WMA

### Read (transcribe video, v0.1.2+)

- Input: video file path (MP4, MOV, M4V, MKV, WebM, AVI)
- Output: JSON envelope on stdout (same as audio transcribe)
- Side effects: spawns ffmpeg subprocess; writes temp WAV to
  `$TMPDIR`; writes to model cache directory
- Idempotent: yes
- Requires: ffmpeg 4.0+ on PATH or via `--ffmpeg-binary`
- New error variants: `VideoExtractionFailed` (exit 65),
  `FfmpegNotFound` (exit 69), `UnsupportedVideoFormat` (exit 65)
- New CLI flags: `--ffmpeg-binary <PATH>`,
  `--no-ffmpeg-fallback`
- New env vars: `WHISPER_FFMPEG_BINARY`,
  `WHISPER_NO_FFMPEG_FALLBACK`

### Discovery

- Input: none
- Output: command tree, JSON Schema, or configuration JSON
- Side effects: none
- Idempotent: yes

### List (models)

- Input: optional model name
- Output: array of models with size, description, downloaded flag
- Side effects: none
- Idempotent: yes

### Mutate (models download or remove)

- Input: model name, optional dry-run flag
- Output: status envelope with action, path, optional etag
- Side effects: writes or deletes file in model cache directory
- Idempotent: yes (download is no-op if already cached)

## JSON Consumption Pattern

The envelope is the unit of consumption. Each line of stdout is a
complete, parseable JSON object. NDJSON streams end with a summary
line containing `"summary": true`.

```bash
whisper-macos-cli transcribe *.ogg --ndjson \
  | jaq -c 'select(.summary | not) | {file, text}'
```

## Error Handling Pattern

When `error: true` is set in the envelope, the agent MUST treat the
result as a failure and may use the `retryable` flag plus
`retry_after_ms` to decide whether to retry.

```python
output = json.loads(result.stdout)
if output.get("error"):
    if output["retryable"]:
        time.sleep(output["retry_after_ms"] / 1000)
        # retry
    else:
        # log and surface error to user
        pass
```

## Self-Describing Pattern

The CLI can scaffold its own skill descriptor into any target
directory:

```bash
whisper-macos-cli init --target /path/to/agent/workspace
```

This writes `SKILL.md` and `AGENTS.md` into the target directory
with the contract, examples, and exit codes.

## Exit Code Reference

| Code | Meaning                | Retryable |
|------|------------------------|-----------|
| 0    | Success                | n/a       |
| 2    | Usage error            | no        |
| 64   | No input provided      | no        |
| 65   | Invalid audio or video data, video extraction failed, unsupported video format | no |
| 66   | Input file not found    | no        |
| 69   | Service unavailable (ffmpeg missing or download failed) | yes |
| 70   | Inference error         | no        |
| 74   | I/O error               | no        |
| 78   | Configuration error     | no        |
| 130  | SIGINT (Ctrl+C)         | no        |
| 141  | Broken pipe             | no        |
| 143  | SIGTERM                 | no        |

## Composition with Unix Tools

- `xh` for HTTP downloads
- `fd` for file discovery
- `bat` for syntax-highlighted preview
- `jaq` for JSON querying
- `ripgrep` for text search
- `xargs` for parallel dispatch
- `timeout` for bounded execution
- `procs` for process inspection
