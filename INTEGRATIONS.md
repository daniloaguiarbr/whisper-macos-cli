[English version](INTEGRATIONS.md) | [Versão em Português Brasileiro](INTEGRATIONS.pt-BR.md)

> Twelve AI coding assistants, twelve AI frameworks, twelve Unix composable CLIs — verified integrations.

# Integrations

whisper-macos-cli integrates with 35+ tools and platforms. The CLI is
designed to behave like any other Unix tool, so integration is
invariably via stdin/stdout, environment variables, or one of the
documented subcommands.

## Flag Aliases and New Flags by Version

### v0.1.0 (current)

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
| Composable CLIs         | 11    | xh, fd, bat, jaq, ripgrep, xargs           |
| CI/CD Platforms         | 4     | GitHub Actions, GitLab CI, Buildkite      |
| MCP Servers             | 3     | whisper-macos-cli, WhatsApp, Slack        |
| Total                   | 42    |                                           |

## AI Coding Assistants

- Claude Code
- OpenCode
- Codex CLI
- Gemini CLI
- Cline
- Cursor
- Windsurf
- Aider
- Continue
- Cody (Sourcegraph)
- Tabnine
- Replit Agent

## AI Frameworks and Runtimes

- LangChain
- LlamaIndex
- Haystack
- Semantic Kernel
- AutoGen
- CrewAI
- smolagents
- PydanticAI
- Atomic Agents
- DSPy
- Guidance
- Outlines

## Composable CLIs in the Project Ecosystem

- xh (HTTP client for streaming downloads)
- fd (file discovery for batch)
- bat (syntax-highlighted preview)
- jaq (JSON querying in pipelines)
- ripgrep (text search across outputs)
- xargs (parallel dispatch)
- timeout (bounded execution)
- procs (process inspection)
- ouch (archive extraction)
- dutree (disk usage visualization)
- dysk (filesystem inspection)

## MCP Servers

- whisper-macos-cli MCP wrapper (planned, off by default)
- WhatsApp MCP (planned integration)
- Slack MCP (planned integration)

## CI/CD Platforms

- GitHub Actions (matrix + signing)
- GitLab CI
- CircleCI
- Buildkite

## Notes

- All AI agents integrate by invoking the binary via subprocess
- All composable CLIs are tested in the cookbook recipes
- MCP exposure is opt-in via the `mcp` feature flag
- CI/CD integration uses Trusted Publishing via OIDC
