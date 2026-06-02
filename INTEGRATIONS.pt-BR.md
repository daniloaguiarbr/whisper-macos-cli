[English version](INTEGRATIONS.md) | [Versão em Português Brasileiro](INTEGRATIONS.pt-BR.md)

> Doze assistentes de codificação IA, doze frameworks de IA, onze CLIs Unix componíveis — integrações verificadas.

# Integrações

whisper-macos-cli integra com 35+ ferramentas e plataformas. A CLI é
projetada para se comportar como qualquer outra ferramenta Unix, então
a integração é invariavelmente via stdin/stdout, variáveis de
ambiente ou um dos subcomandos documentados.

## Aliases de Flags e Novas Flags por Versão

### v0.1.0 (atual)

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
| CLIs Unix Componíveis    | 11       | xh, fd, bat, jaq, ripgrep, xargs           |
| Plataformas de CI/CD     | 4        | GitHub Actions, GitLab CI, Buildkite      |
| Servidores MCP           | 3        | whisper-macos-cli, WhatsApp, Slack        |
| Total                   | 42       |                                           |

## Assistentes de Codificação IA

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

## Frameworks e Runtimes de IA

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

## CLIs Unix Componíveis no Ecossistema do Projeto

- xh (cliente HTTP para downloads em stream)
- fd (descoberta de arquivos para lote)
- bat (preview com syntax highlighting)
- jaq (query de JSON em pipelines)
- ripgrep (busca de texto em saídas)
- xargs (dispatch paralelo)
- timeout (execução com limite)
- procs (inspeção de processos)
- ouch (extração de arquivos)
- dutree (visualização de uso de disco)
- dysk (inspeção de filesystems)

## Servidores MCP

- Wrapper MCP do whisper-macos-cli (planejado, off por padrão)
- WhatsApp MCP (integração planejada)
- Slack MCP (integração planejada)

## Plataformas de CI/CD

- GitHub Actions (matrix + signing)
- GitLab CI
- CircleCI
- Buildkite

## Notas

- Todos os agentes de IA integram invocando o binário via subprocess
- Todas as CLIs componíveis são testadas nas receitas do cookbook
- Exposição MCP é opt-in via flag `mcp`
- Integração CI/CD usa Trusted Publishing via OIDC
