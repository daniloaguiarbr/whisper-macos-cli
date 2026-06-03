[English version](AGENTS.md) | [Versão em Português Brasileiro](AGENTS.pt-BR.md)

# AGENTS — Guia de Integração para Agentes

Este documento é escrito para agentes de IA e ferramentas de orquestração
que precisam invocar whisper-macos-cli como serviço de transcrição
caixa-preta.

## Contrato

- Um objeto JSON por arquivo no stdout
- correlation_id é um UUID v7 gerado por invocação de processo
- schema_version reflete a versão do envelope
- stderr carrega logs de tracing que podem ser suprimidos com --quiet
- Códigos de saída seguem convenção sysexits.h

## Envelope JSON de Saída

```json
{
  "schema_version": "0.1.0",
  "correlation_id": "0190a3b4-7c8d-7abc-9def-1234567890ab",
  "file": "voz.ogg",
  "language": "pt",
  "language_source": "os_locale",
  "model": "large-v3",
  "duration_seconds": 45.2,
  "text": "Texto transcrito completo aqui",
  "vad_chunks": 3,
  "processing_time_ms": 8432
}
```

## Envelope de Erro

```json
{
  "schema_version": "0.1.0",
  "error": true,
  "code": 66,
  "message": "input not found: /tmp/ausente.ogg",
  "category": "input",
  "retryable": false,
  "retry_after_ms": null,
  "hint": "verifique o caminho do arquivo",
  "docs_url": "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.md",
  "correlation_id": "0190a3b4-7c8d-7abc-9def-1234567890ab"
}
```

## Início Rápido para Agentes

```bash
# Setup em 30 segundos
cargo install whisper-macos-cli
whisper-macos-cli models download

# Transcrever áudio
whisper-macos-cli transcribe voz.ogg

# Transcrever vídeo (requer ffmpeg)
brew install ffmpeg
whisper-macos-cli transcribe video.mp4
```

## Orçamento de Tokens

- Sobrecarga por invocação: 200 tokens (parsing do envelope)
- Por arquivo transcrito: 50 tokens + comprimento do texto
- Tratamento de erro: 100 tokens

## Códigos de Saída

| Código | Significado                               | Retentável |
|--------|-------------------------------------------|------------|
| 0      | Sucesso                                   | n/a        |
| 2      | Erro de uso                               | não        |
| 64     | Nenhuma entrada                           | não        |
| 65     | Áudio/vídeo inválido                      | não        |
| 66     | Arquivo não encontrado                    | não        |
| 69     | Serviço indisponível (ffmpeg ausente)     | sim        |
| 70     | Erro de inferência                        | não        |
| 74     | Erro de I/O                               | não        |
| 78     | Erro de configuração                      | não        |
| 130    | SIGINT (Ctrl+C)                           | não        |
| 141    | Pipe quebrado                             | não        |
| 143    | SIGTERM                                   | não        |

## Transcrição de Vídeo (v0.1.2+)

Desde a v0.1.2, o whisper-macos-cli suporta containers de vídeo
(MP4, MOV, M4V, MKV, WebM, AVI) extraindo a trilha de áudio via
subprocess ffmpeg. Requer ffmpeg 4.0+ no PATH.

```bash
whisper-macos-cli transcribe video.mp4
```

Áudio OGG/Opus do WhatsApp que falha no decoder nativo é
automaticamente roteado via ffmpeg como fallback transparente.

Veja [docs/VIDEO-EXTRACTION.pt-BR.md](docs/VIDEO-EXTRACTION.pt-BR.md)
para detalhes completos.

## Subcomandos para Descoberta de Agentes

```bash
# Árvore completa de comandos em JSON
whisper-macos-cli commands --format json

# JSON Schema completo do envelope de saída
whisper-macos-cli schema

# Configuração efetiva
whisper-macos-cli config

# Scaffold de skill auto-descritiva
whisper-macos-cli init --target /caminho/do/workspace
```

## Agentes de IA Compatíveis

Claude Code, OpenCode, Codex CLI, Gemini CLI, Cline, Cursor,
Windsurf, Aider, Continue, Cody, Tabnine, Replit Agent. Lista
completa em [INTEGRATIONS.md](INTEGRATIONS.md).

## Documentação para Integração Profunda

Veja [docs/AGENTS.md](docs/AGENTS.md) para o guia completo de autor
de agente incluindo padrões de integração, contratos de crate e
operações CRUD.
