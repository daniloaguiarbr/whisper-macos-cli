[English version](docs/TROUBLESHOOTING.md) | [Versão em Português Brasileiro](docs/TROUBLESHOOTING.pt-BR.md)

# Troubleshooting

## exit code 64 — no input

You did not pass any file and stdin is a TTY.

Fix: pass a file argument or pipe from stdin.

```bash
whisper-macos-cli transcribe audio.ogg
cat audio.ogg | whisper-macos-cli transcribe
```

## exit code 65 — invalid audio data

The audio file is corrupt, encrypted, or uses an unsupported codec.

Fix: verify the file plays in a media player, then re-export it as
uncompressed WAV or standard OGG/Opus.

## exit code 66 — input file not found

The path you provided does not exist or is not readable.

Fix: check the path. Use `ls` to verify the file is present.

## exit code 69 — service unavailable

Either the model download failed or you are on an unsupported
platform.

Fix:

1. Run `whisper-macos-cli doctor` to see what's wrong
2. Check your network connection
3. Verify you are on macOS with Apple Silicon

## exit code 70 — whisper inference failed

The Whisper model encountered an internal error.

Fix: try a smaller model with `--model base`.

## exit code 74 — I/O error

A low-level I/O failure occurred.

Fix: check disk space, file permissions, and that no other process
holds an exclusive lock on the target file.

## exit code 78 — configuration error

The model is not downloaded or the configuration is invalid.

Fix: run `whisper-macos-cli models download` to install the model.

## Audio Decode

If you see `audio decode failed: probe failed`, the file may be
encrypted (DRM) or use a codec the decoder does not recognize. Run
`whisper-macos-cli doctor` and check the audio format list.

## Model Download

If a model download is interrupted, the temp file (`.bin.tmp`) is
automatically cleaned up. Re-run `whisper-macos-cli models download`
to retry. Up to 3 retries with exponential backoff are attempted.

## Inference Latency

The default `large-v3` model requires approximately 2 GB of unified
memory. On M1, the first inference takes 2-5 seconds for warmup;
subsequent inferences are faster. Use `--model small` for 5x speedup
at the cost of accuracy.

## Air-Gapped Mode

When network access is unavailable, pre-download models and run
with `--offline` to skip the network check. The doctor command will
report network as `fail` but the CLI will still work for local
transcription.

## SIGINT During Long Transcription

Pressing Ctrl+C twice within 1.5 seconds forces immediate exit.
The first Ctrl+C allows in-flight work to complete cleanly.
