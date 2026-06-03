# Video Extraction — whisper-macos-cli v0.1.2+

Since v0.1.2, whisper-macos-cli can transcribe audio from video
containers. The video file's audio track is extracted to a temporary
WAV via ffmpeg, then fed to the regular whisper.cpp pipeline.

## Supported Formats

| Container | Magic bytes           | Notes                        |
|-----------|-----------------------|------------------------------|
| MP4       | `....ftypisom`        | Most common; YouTube exports  |
| MOV       | `....ftypqt  `        | Apple QuickTime              |
| M4V       | `....ftypM4V `        | iTunes video                 |
| MKV       | `0x1A 0x45 0xDF 0xA3` | Matroska                     |
| WebM      | `0x1A 0x45 0xDF 0xA3` | Matroska-derived             |
| AVI       | `RIFF....AVI `        | Legacy Windows                |
| M4A       | `....ftypM4A `        | Audio-only MPEG-4 (often)    |
| FLV       | `FLV\x01`             | Flash Video                  |
| WMV/WMA   | `0x30 0x26 0xB2 0x75` | ASF container                |

Detection uses magic bytes first, then file extension. Renamed files
(`.ogg` with MP4 magic) are still routed correctly.

## Requirements

- **ffmpeg 4.0 or later** must be installed and accessible on `PATH`,
  or its location must be specified via `--ffmpeg-binary` or the
  `WHISPER_FFMPEG_BINARY` environment variable.

### Install ffmpeg

- **macOS:** `brew install ffmpeg`
- **Ubuntu/Debian:** `sudo apt-get install ffmpeg`
- **Windows (Chocolatey):** `choco install ffmpeg`
- **Windows (winget):** `winget install Gyan.FFmpeg`

## Usage

### Transcribe a video file

```bash
whisper-macos-cli transcribe video.mp4
```

The output is a single JSON envelope (or NDJSON line in batch mode)
identical to a regular audio transcription. The `file` field contains
the video filename.

### Batch transcription of a folder of videos

```bash
whisper-macos-cli transcribe --ndjson --concurrency 4 *.mp4
```

### Specify a custom ffmpeg binary

```bash
whisper-macos-cli transcribe --ffmpeg-binary /opt/local/bin/ffmpeg video.mov
```

Or via environment:

```bash
export WHISPER_FFMPEG_BINARY=/opt/local/bin/ffmpeg
whisper-macos-cli transcribe video.mkv
```

### Disable ffmpeg fallback entirely

If you want to test that the native symphonia decoder is sufficient
(reproducing the OGG/Opus bug, for instance), you can disable the
fallback:

```bash
whisper-macos-cli transcribe --no-ffmpeg-fallback audio.ogg
```

When this flag is set and a video file is supplied, the CLI returns
`Error::UnsupportedVideoFormat` (exit 65) instead of attempting
extraction.

## OGG/Opus Auto-Fallback

The OGG/Opus files produced by WhatsApp (and other voice messengers)
trigger a known bug in the `symphonia` crate
([Issue #8](https://github.com/pdeljanov/Symphonia/issues/8)) — the
"Opus" status is officially listed as **"In work"** by the project.
As of v0.1.2, whisper-macos-cli transparently detects this failure
and re-runs the decode through ffmpeg, which handles the codec
correctly. The fallback is automatic and produces identical output
to a successful native decode.

To verify the fallback is happening, run with `-v`:

```bash
whisper-macos-cli transcribe -v audio.ogg
# stderr: ... native decode failed, attempting ffmpeg fallback
```

## ffmpeg Not Found

If ffmpeg is not installed and the input is a video (or the native
decode fails), the CLI returns:

```json
{
  "schema_version": "0.1.2",
  "error": true,
  "code": 69,
  "message": "ffmpeg not found in PATH: install via `brew install ffmpeg` or set --ffmpeg-binary",
  "category": "service",
  "retryable": false,
  "retry_after_ms": null,
  "hint": "install ffmpeg via `brew install ffmpeg` or set --ffmpeg-binary",
  "docs_url": "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/VIDEO-EXTRACTION.md#ffmpeg-not-found",
  "correlation_id": "..."
}
```

Exit code 69. Fix by installing ffmpeg (see above) and retrying.

## Security and Process Isolation

The ffmpeg subprocess is hardened with the following guarantees:

- **`env_clear()`** — no host environment variables are inherited
  except a minimal allowlist (`PATH`, `HOME`, `TMPDIR`, `LANG`,
  `LC_ALL`). Secrets like `*_TOKEN` cannot leak into ffmpeg logs.
- **`setsid()` on Unix / `CREATE_NEW_PROCESS_GROUP` on Windows** —
  the child runs in its own process group. Ctrl+C delivered to the
  parent does not silently propagate to ffmpeg.
- **Kill-on-drop** — the child handle is wrapped in a `SafeChild`
  guard. If the parent panics, the child is killed (SIGKILL on Unix,
  TerminateProcess on Windows) to prevent zombie processes.
- **Bounded timeout** — default 180s. On timeout, the child is killed
  and `Error::VideoExtractionFailed` is returned.
- **Output validation** — the extracted WAV is validated post-process:
  must have `RIFF...WAVE` header, size must match the RIFF chunk
  size. Catches the "ffmpeg exit 0 but empty file" class of bugs.
- **Temp cleanup** — the temp WAV is removed via a `Drop` guard
  even if decode panics.

## Limits

- **Maximum duration:** 24 hours (inherited from the audio pipeline).
- **Maximum file size:** limited by the temp directory; ~3 GB
  practical ceiling for 1h of typical video.
- **Concurrency:** the `--concurrency N` flag governs how many
  transcriptions run in parallel, each of which may spawn an
  ffmpeg subprocess. On 4-core machines, `--concurrency 2` is safe.

## Exit Codes

| Code | Meaning                                       |
|------|-----------------------------------------------|
| 0    | Success                                       |
| 2    | Usage error (invalid arguments)               |
| 64   | No input provided                             |
| 65    | Invalid data (corrupt audio, video extraction failed, unsupported format) |
| 66   | Input file not found                          |
| 69   | Service unavailable (ffmpeg missing, model download failed) |
| 70   | Inference error                               |
| 74   | I/O error                                     |
| 78   | Configuration error                           |

## See Also

- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — general diagnosis
- [SKILL.md](../SKILL.md) — JSON contract reference
- [AGENTS.md](../AGENTS.md) — agent integration guide
- [CHANGELOG.md](../CHANGELOG.md) — release notes
