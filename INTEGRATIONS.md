[English version](INTEGRATIONS.md) | [Versão em Português Brasileiro](INTEGRATIONS.pt-BR.md)

> Twelve AI coding assistants, twelve AI frameworks, thirty-nine verified integrations.

# Integrations

whisper-macos-cli integrates with 39 tools and platforms. The CLI
is designed to behave like any other Unix tool, so integration is
invariably via stdin/stdout, environment variables, or one of the
documented subcommands.

## Flag Aliases and New Flags by Version

### v0.1.2

- `--ffmpeg-binary <PATH>` flag (env: `WHISPER_FFMPEG_BINARY`) for
  video container audio extraction
- `--no-ffmpeg-fallback` flag (env: `WHISPER_NO_FFMPEG_FALLBACK`)
  to disable the OGG/Opus transparent fallback
- Video transcription support: MP4, MOV, M4V, MKV, WebM, AVI

### v0.1.0

- `WHISPER_MODEL` environment variable as alias for `--model`
- `WHISPER_LANGUAGE` environment variable as alias for `--language`
- `CI=true` environment variable as alias for `--no-input`
- `--print-schema` global flag for JSON Schema
- `--print-config` global flag for effective configuration
- `commands` subcommand for full command tree as JSON
- `init` subcommand for skill scaffold generation

## Summary Table

| Category                | Count | Examples                                  |
|-------------------------|-------|-------------------------------------------|
| AI Coding Assistants    | 12    | Claude Code, Cursor, Aider, Cody          |
| AI Frameworks           | 12    | LangChain, LlamaIndex, DSPy               |
| Composable CLIs         | 11    | xh, fd, bat, jaq, ripgrep, xargs          |
| CI/CD Platforms         | 4     | GitHub Actions, GitLab CI, Buildkite      |
| Total                   | 39    |                                           |

## Claude Code

Integration via subprocess. The JSON envelope on stdout is parsed
directly. See root `AGENTS.md` for the full JSON contract.

## OpenCode

Native integration via `SKILL.md`. The agent auto-discovers the
skill.

## Codex CLI (OpenAI)

Integration via subprocess. Use `--quiet --no-input` for
pipeline invocations.

## Gemini CLI (Google)

Integration via subprocess. Accepts the JSON envelope output.

## Cline

Integration via subprocess. Supports NDJSON for batch.

## Cursor

Integration via subprocess. Configure the binary path in
Settings > Custom Commands.

## Windsurf

Integration via subprocess. Configure as external shell command.

## Aider

Integration via subprocess. Use `--quiet` to suppress stderr.

## Continue

Integration via subprocess. Configurable in
`~/.continue/config.json`.

## Cody (Sourcegraph)

Integration via subprocess. Use `--ndjson` for batch.

## Tabnine

Integration via subprocess. Configure as external command.

## Replit Agent

Integration via subprocess. Use `--quiet` for clean output.

## LangChain

Integration via subprocess within tool nodes. Use Python
`subprocess.run` with `capture_output=True`.

## LlamaIndex

Integration via subprocess in custom tools. Use `SubprocessTool`.

## Haystack

Integration via subprocess in custom components. Use
`Subprocess.run`.

## Semantic Kernel

Integration via subprocess in native functions. Configure as
external process.

## AutoGen

Integration via subprocess in assistant agents. Use
`subprocess_run`.

## CrewAI

Integration via subprocess in custom tools. Use `BaseTool` with
subprocess backend.

## smolagents

Integration via subprocess in custom tools. Use `tool` decorator.

## PydanticAI

Integration via subprocess in tools. Use `subprocess.run` for
synchronous invocation.

## Atomic Agents

Integration via subprocess in custom tools. Use `BaseAgent` with
subprocess backend.

## DSPy

Integration via subprocess in custom modules. Use `dspy.Tool` with
`subprocess.run`.

## Guidance

Integration via subprocess in custom guidance programs. Use
`subprocess.run` for batch.

## Outlines

Integration via subprocess in custom generators. Use
`subprocess.run` for local inference.

## GitHub Actions

Integration via subprocess in CI. Use `runs-on: macos-14` with
`actions/checkout` and `cargo install whisper-macos-cli`.

## GitLab CI

Integration via subprocess. Configure macOS Apple Silicon runner
with Rust toolchain.

## CircleCI

Integration via subprocess via `macos.x86` or `arm64` executor.

## Buildkite

Integration via subprocess via macOS Apple Silicon agent.

## Notes

- All AI agents integrate by invoking the binary via subprocess
- All composable CLIs are tested in the cookbook recipes
- CI/CD integration uses Trusted Publishing via OIDC
- Subprocess integration is stable and versioned via
  `schema_version`
