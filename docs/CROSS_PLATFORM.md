[English version](docs/CROSS_PLATFORM.md) | [Versão em Português Brasileiro](docs/CROSS_PLATFORM.pt-BR.md)

> The pain you already know: a CLI that runs on one OS and pretends to be portable.

# Cross-Platform Support

## The Pain You Already Know

You want to ship a transcription skill to your users. Half of them
have macOS Apple Silicon, the other half have Intel macs, Windows
machines, or Linux servers. You cannot ship a single binary that
works everywhere because whisper.cpp's Metal backend is macOS-only.

## Why

- whisper.cpp's GPU acceleration requires Apple Metal
- CPU-only fallback on Linux or Windows is 50x slower
- Cross-compiling Metal binaries is not supported by Apple
- Forcing portability sacrifices the quality-of-life the project is
  built around

## Sovereignty

- macOS Apple Silicon is the primary target
- Other platforms are explicitly excluded, not accidentally broken
- Users on other platforms are routed to upstream whisper.cpp

## Support Matrix

| Target                    | Tier   | Status         |
|---------------------------|--------|----------------|
| aarch64-apple-darwin      | Tier 1 | Full support   |
| x86_64-apple-darwin       | Tier 2 | Compiles only  |
| x86_64-unknown-linux-gnu  | None   | Not supported  |
| aarch64-unknown-linux-gnu | None   | Not supported  |
| x86_64-pc-windows-msvc   | None   | Not supported  |

## macOS Notes

- Apple Silicon is required because whisper.cpp's Metal backend is
  the only GPU acceleration path
- macOS 13 (Ventura) or later is required for Metal 3
- Xcode Command Line Tools provide the Metal compiler
- The default model loads into unified memory

## Linux Notes

- Not supported
- The Metal GPU backend has no Linux implementation
- CPU-only whisper.cpp is 50x slower than Metal
- Users on Linux should use upstream whisper.cpp or faster-whisper

## Windows Notes

- Not supported
- Same reason as Linux
- Windows builds would require a separate whisper.cpp fork
- Users on Windows should use WSL with Linux whisper.cpp

## Container Notes

- Not published as a container
- macOS containers do not exist for production use
- Use a macOS host with a Rust toolchain

## Shell Support

The CLI is tested on:

- bash 5.x
- zsh 5.x
- fish 3.x
- nushell 0.80+

## File Paths and XDG

The model cache directory is:

- macOS: `~/Library/Application Support/whisper-macos-cli/models/`
- Linux: `~/.local/share/whisper-macos-cli/models/` (if supported)
- Windows: `%APPDATA%\whisper-macos-cli\models\` (if supported)

## Performance by Target

- aarch64-apple-darwin (M1): 1x real-time
- aarch64-apple-darwin (M2 Pro): 0.5x real-time
- aarch64-apple-darwin (M3 Max): 0.3x real-time
- x86_64-apple-darwin (Intel): 5x real-time (CPU only)

## Agents Validated per Platform

- macOS Apple Silicon: Claude Code, OpenCode, Codex CLI, Gemini CLI,
  Cline, Cursor, Windsurf, Aider, Continue, Cody, Tabnine, Replit Agent
- All other platforms: not validated, not supported
