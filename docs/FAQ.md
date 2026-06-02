[English version](docs/FAQ.md) | [Versão em Português Brasileiro](docs/FAQ.pt-BR.md)

# Frequently Asked Questions

## What is whisper-macos-cli?

A Rust CLI that transcribes audio files locally on macOS Apple
Silicon using whisper.cpp with Metal GPU acceleration. It is
designed for integration with AI agents and Unix pipelines via a
strict stdin/stdout JSON contract.

## Why macOS-only?

The whisper.cpp Metal GPU backend requires Apple's Metal framework,
which is only available on macOS. Cross-platform support is not a
goal. For cross-platform transcription, use the original
[whisper.cpp](https://github.com/ggml-org/whisper.cpp) or
[faster-whisper](https://github.com/SYSTRAN/faster-whisper) projects.

## Why is the default model large-v3?

Quality. The `large-v3` model produces the most accurate
transcriptions, especially for non-English languages. The first
download is ~3 GB; subsequent runs use the cached file.

## Can I use this for WhatsApp voice messages?

Yes. WhatsApp voice messages are encoded as OGG/Opus. The CLI
handles them natively and discards the 80 ms pre-skip
automatically.

## Does it work offline?

Yes, after the model is downloaded. Use `--offline` to skip network
checks.

## Does it phone home?

No. The only network activity is the model download from
huggingface.co. See `PRIVACY.md` for the full policy.

## Why JSON on stdout?

JSON is the lingua franca of AI agents. By emitting structured JSON
with a stable schema and a `correlation_id`, agents can parse
results reliably and trace requests across services.

## How do I update the model?

```bash
whisper-macos-cli models remove large-v3
whisper-macos-cli models download large-v3
```

## Can I run multiple models in parallel?

The CLI loads a single model per process. Run multiple CLI instances
in parallel for multi-model workflows. The `--concurrency` flag
controls parallel transcriptions within a single model.

## How accurate is it?

For Portuguese (pt-BR) and English, accuracy is comparable to
OpenAI Whisper large-v3 with WER typically under 5% on clean audio.

## What about privacy of my audio?

Audio is processed entirely on your local machine. Nothing is
transmitted to any external service. See `PRIVACY.md`.

## How do I report a bug?

Open an issue at
https://github.com/daniloaguiarbr/whisper-macos-cli/issues using
the bug report template.

## Where do I report a security vulnerability?

Through GitHub Security Advisories at
https://github.com/daniloaguiarbr/whisper-macos-cli/security/advisories/new
— NOT a public issue.
