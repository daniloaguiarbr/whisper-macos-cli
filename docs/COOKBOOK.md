[English version](docs/COOKBOOK.md) | [Versão em Português Brasileiro](docs/COOKBOOK.pt-BR.md)

> Twenty production-grade recipes covering ingestion, recovery, and audit patterns.

# Cookbook

## Latency Note

- Cold start with large-v3: 2-5 seconds warmup
- Subsequent transcriptions: roughly real-time on M2 Pro
- Concurrency scales linearly up to 8 on M1 Pro

## Default Values Reference

| Parameter        | Default    | Override                |
|------------------|------------|-------------------------|
| Model            | large-v3   | `--model`, `WHISPER_MODEL` |
| Language         | OS locale  | `--language`, `WHISPER_LANGUAGE` |
| Beam size        | 8          | `--beam-size`           |
| VAD threshold    | 0.5        | `--vad-threshold`       |
| Concurrency      | 2          | `--concurrency`         |
| Output format    | JSON       | `--ndjson`              |

## How To Transcribe a Single WAV File

```bash
whisper-macos-cli transcribe speech.wav
```

```json
{"schema_version":"0.1.0","correlation_id":"...","file":"speech.wav","language":"pt","language_source":"os_locale","model":"large-v3","duration_seconds":12.4,"text":"Olá mundo","vad_chunks":1,"processing_time_ms":1820}
```

## How To Transcribe a WhatsApp Voice Message (OGG/Opus)

```bash
whisper-macos-cli transcribe voice-message.ogg
```

Opus pre-skip (3840 samples at 48kHz) is automatically discarded.

## How To Transcribe from stdin

```bash
cat recording.ogg | whisper-macos-cli transcribe
```

Stdin is capped at 2 GB to prevent OOM.

## How To Batch Transcribe as NDJSON

```bash
whisper-macos-cli transcribe *.ogg --ndjson --concurrency 4
```

Each file emits a JSON object on stdout. A final `{"summary": true, ...}`
line reports totals.

## How To Force a Language

```bash
whisper-macos-cli transcribe --language pt audio.wav
```

## How To Use a Specific Model

```bash
whisper-macos-cli models download small
whisper-macos-cli transcribe --model small audio.wav
```

## How To Set Custom Beam Size

```bash
whisper-macos-cli transcribe --beam-size 4 audio.wav
```

Valid range: 1-16. Higher is slower but more accurate.

## How To Get Timestamped Segments

```bash
whisper-macos-cli transcribe --timestamps audio.wav
```

Adds `segments` array with `start`, `end`, `text` per segment.

## How To Disable VAD

```bash
whisper-macos-cli transcribe --vad-threshold 0.0 audio.wav
```

Threshold 0.0 effectively disables VAD, transcribing the full audio
without speech segmentation.

## How To Run With Maximum Verbosity

```bash
whisper-macos-cli -vvv transcribe audio.wav
```

## How To Run Silently

```bash
whisper-macos-cli --quiet transcribe audio.wav
```

Suppresses all stderr output.

## How To List Available Models

```bash
whisper-macos-cli models list
```

## How To Download the Default Model

```bash
whisper-macos-cli models download
```

Downloads `large-v3` (~3 GB).

## How To Download a Specific Model

```bash
whisper-macos-cli models download small
```

## How To Print the Model File Path

```bash
whisper-macos-cli models path small
```

## How To Remove a Model (dry run)

```bash
whisper-macos-cli models remove tiny --dry-run
```

## How To Remove a Model (real)

```bash
whisper-macos-cli models remove tiny
```

## How To Diagnose the Environment

```bash
whisper-macos-cli doctor
```

Returns exit code 0 if all checks pass, 78 otherwise.

## How To Get the JSON Schema

```bash
whisper-macos-cli schema
```

Returns the full envelope schema including agentNotes, invariants,
sideEffects, idempotent, checkpointable, and tokenBudget.

## How To Get the Effective Configuration

```bash
whisper-macos-cli config
```

Returns the current effective configuration as JSON.

## How To Validate Inputs Without Transcribing

```bash
whisper-macos-cli transcribe --dry-run audio.ogg
```

Resolves inputs, model, and language without loading the model or
running inference.

## How To Use in CI/CD

```bash
CI=true whisper-macos-cli transcribe --quiet --no-input \
  --language en audio.ogg > result.json
```

`CI=true` disables interactive prompts. `--no-input` is honored
automatically. `--quiet` suppresses stderr.

## How To Transcribe a Video File

```bash
whisper-macos-cli transcribe video.mp4
```

Requires ffmpeg 4.0+ on PATH. See
[VIDEO-EXTRACTION.md](VIDEO-EXTRACTION.md) for the full list of
supported containers and security guarantees.

## How To Batch Transcribe a Folder of Videos

```bash
whisper-macos-cli transcribe --ndjson --concurrency 2 *.mp4
```

Concurrency is limited on video workflows because each
transcription spawns an ffmpeg subprocess. On 4-core machines,
`--concurrency 2` is safe.

## How To Use a Custom ffmpeg Binary

```bash
whisper-macos-cli transcribe --ffmpeg-binary /opt/local/bin/ffmpeg video.mov
```

Use this when ffmpeg is installed via MacPorts, Homebrew, or
a custom prefix and not on the default PATH.

## How To Reproduce a Native Decoder Bug

```bash
whisper-macos-cli transcribe --no-ffmpeg-fallback audio.ogg
```

Disables the transparent ffmpeg fallback. Use this when
debugging the native symphonia decoder behavior.

## How To Verify the ffmpeg Fallback is Happening

```bash
whisper-macos-cli transcribe -v audio.ogg
# stderr: ... native decode failed, attempting ffmpeg fallback
```

The `-v` flag increases verbosity so the fallback decision is
logged to stderr.
