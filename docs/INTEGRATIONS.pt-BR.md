[English version](README.md) | [Versão em Português Brasileiro](README.pt-BR.md)

> Doze assistentes de codificação IA, doze frameworks de IA, trinta e nove integrações verificadas via Unix pipelines.

# Integrações

whisper-macos-cli integra com 39 ferramentas e plataformas. A CLI
foi projetada para se comportar como qualquer outra ferramenta
Unix, então a integração é invariavelmente via stdin/stdout,
variáveis de ambiente ou um dos subcomandos documentados.

## Aliases de Flags e Novas Flags por Versão

### v0.1.2

- Flag `--ffmpeg-binary <PATH>` (env: `WHISPER_FFMPEG_BINARY`) para
  extrair áudio de containers de vídeo
- Flag `--no-ffmpeg-fallback` (env: `WHISPER_NO_FFMPEG_FALLBACK`)
  para desabilitar o fallback OGG/Opus via ffmpeg
- Suporte a transcrição de vídeo: MP4, MOV, M4V, MKV, WebM, AVI
- Auto-fallback transparente para WhatsApp OGG/Opus

### v0.1.0

- Variável de ambiente `WHISPER_MODEL` como alias para `--model`
- Variável de ambiente `WHISPER_LANGUAGE` como alias para `--language`
- Variável de ambiente `CI=true` como alias para `--no-input`
- Flag global `--print-schema` para JSON Schema
- Flag global `--print-config` para configuração efetiva
- Subcomando `commands` para árvore completa de comandos em JSON
- Subcomando `init` para geração de scaffold de skill

## Tabela Resumo

| Categoria                 | Contagem | Exemplos                                  |
|--------------------------|----------|-------------------------------------------|
| Assistentes de Codificação IA | 12    | Claude Code, Cursor, Aider, Cody          |
| Frameworks de IA         | 12       | LangChain, LlamaIndex, DSPy               |
| CLIs Unix Componíveis    | 11       | xh, fd, bat, jaq, ripgrep, xargs          |
| Plataformas de CI/CD     | 4        | GitHub Actions, GitLab CI, Buildkite      |
| Total                   | 39       |                                           |

## Claude Code

Integração via subprocess. O JSON envelope no stdout é parseado
diretamente. Veja `AGENTS.md` raiz para o contrato JSON completo.

## OpenCode

Integração nativa via SKILL.md. O agente descobre a skill
automaticamente.

## Codex CLI (OpenAI)

Integração via subprocess. Use `--quiet --no-input` para chamadas
em pipeline.

## Gemini CLI (Google)

Integração via subprocess. Aceita saída JSON do envelope.

## Cline

Integração via subprocess. Suporta NDJSON para batch.

## Cursor

Integração via subprocess. Configure o caminho do binário em
Settings > Custom Commands.

## Windsurf

Integração via subprocess. Configure como shell command externo.

## Aider

Integração via subprocess. Use `--quiet` para suprimir stderr.

## Continue

Integração via subprocess. Configurável em `~/.continue/config.json`.

## Cody (Sourcegraph)

Integração via subprocess. Use `--ndjson` para batch.

## Tabnine

Integração via subprocess. Configure como external command.

## Replit Agent

Integração via subprocess. Use `--quiet` para output limpo.

## LangChain

Integração via subprocess dentro de tool nodes. Use Python
`subprocess.run` com `capture_output=True`.

## LlamaIndex

Integração via subprocess em custom tools. Use `SubprocessTool`.

## Haystack

Integração via subprocess em custom components. Use
`Subprocess.run`.

## Semantic Kernel

Integração via subprocess em native functions. Configure como
external process.

## AutoGen

Integração via subprocess em assistant agents. Use `subprocess_run`.

## CrewAI

Integração via subprocess em custom tools. Use `BaseTool` com
subprocess backend.

## smolagents

Integração via subprocess em custom tools. Use `tool` decorator.

## PydanticAI

Integração via subprocess em tools. Use `subprocess.run` para
invocação síncrona.

## Atomic Agents

Integração via subprocess em custom tools. Use `BaseAgent` com
subprocess backend.

## DSPy

Integração via subprocess em custom modules. Use `dspy.Tool` com
`subprocess.run`.

## Guidance

Integração via subprocess em custom guidance programs. Use
`subprocess.run` para batch.

## Outlines

Integração via subprocess em custom generators. Use
`subprocess.run` para inferência local.

## GitHub Actions

Integração via subprocess no CI. Use `runs-on: macos-14` com
`actions/checkout` + `cargo install whisper-macos-cli`.

## GitLab CI

Integração via subprocess. Configure runner macOS Apple Silicon
com Rust toolchain.

## CircleCI

Integração via subprocess via `macos.x86` ou `arm64` executor.

## Buildkite

Integração via subprocess via agent macOS Apple Silicon.

## Notas

- Todos os agentes de IA integram invocando o binário via subprocess
- Todas as CLIs componíveis são testadas nas receitas do cookbook
- Integração CI/CD usa Trusted Publishing via OIDC
- A integração por subprocess é estável e versionada via `schema_version`
