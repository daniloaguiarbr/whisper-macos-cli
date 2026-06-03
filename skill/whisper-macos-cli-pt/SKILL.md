---
name: whisper-macos-cli
version: 0.1.2
description: Transcreva arquivos de áudio e vídeo para texto via whisper.cpp com GPU Metal em macOS Apple Silicon. Use quando precisar transcrever áudio, processar mensagens de voz do WhatsApp, transcrever vídeos, converter fala em texto, transcrever lotes de arquivos, construir pipelines de transcrição para agentes de IA, ou sempre que transcrição local for necessária sem serviços em nuvem.
invariants:
  - stdout é sempre JSON válido ou NDJSON
  - stderr é sempre logs legíveis por humanos
  - códigos de saída seguem convenção sysexits.h
  - modelo large-v3 é o padrão
  - OGG/Opus (mensagens de voz do WhatsApp) é suportado nativamente com fallback automático via ffmpeg
  - vídeo é extraído para WAV via ffmpeg subprocess antes da transcrição
triggers:
  - transcrever áudio
  - fala para texto
  - transcrição de áudio
  - transcrição de vídeo
  - whisper.cpp
  - transcrição de mensagem de voz
  - áudio whatsapp
  - transcrição local
  - transcrição em lote
---

# whisper-macos-cli

## Capability

Transcrição local de áudio e vídeo via whisper.cpp com aceleração
Metal GPU em macOS Apple Silicon. Aceita áudio (MP3, OGG/Vorbis,
OGG/Opus/WhatsApp, FLAC, WAV, AAC) e vídeo (MP4, MOV, M4V, MKV,
WebM, AVI). Vídeo é extraído para WAV via ffmpeg subprocess antes
da transcrição. Emite JSON no stdout com texto transcrito.

### Requisitos para vídeo

- ffmpeg 4.0+ deve estar disponível no PATH (ou via `--ffmpeg-binary`)
- Instale com `brew install ffmpeg` no macOS

## Installation

### REQUIRED

- macOS 13 ou superior
- Apple Silicon (M1, M2, M3, M4)
- Xcode Command Line Tools: `xcode-select --install`
- cmake: `brew install cmake`
- Rust 1.88 ou superior: `rustup install stable`

### Correct Pattern

```bash
cargo install whisper-macos-cli
```

## Core Commands

### REQUIRED

- Um objeto JSON por arquivo no stdout
- correlation_id é um UUID v7 gerado por invocação de processo
- schema_version reflete a versão do envelope
- stderr carrega logs de tracing que podem ser suprimidos com --quiet
- Códigos de saída seguem convenção sysexits.h

### Correct Pattern

```bash
# Arquivo de áudio único
whisper-macos-cli transcribe voz.ogg

# Vídeo (extração automática de áudio via ffmpeg)
whisper-macos-cli transcribe video.mp4

# Lote com NDJSON (áudio + vídeo misturados)
whisper-macos-cli transcribe *.ogg *.mp4 --ndjson --concurrency 4

# Via stdin (apenas áudio)
cat audio.mp3 | whisper-macos-cli transcribe

# Especificar ffmpeg customizado
whisper-macos-cli transcribe --ffmpeg-binary /opt/local/bin/ffmpeg video.mov
```

## JSON Contract

### REQUIRED

Toda saída no stdout DEVE ser um objeto JSON válido com no mínimo:
- `schema_version` — string
- `correlation_id` — string (UUID v7)

### Transcription Result

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

### Error Envelope

```json
{
  "schema_version": "0.1.0",
  "error": true,
  "code": 66,
  "message": "input not found",
  "category": "input",
  "retryable": false,
  "retry_after_ms": null,
  "hint": "verifique o caminho do arquivo",
  "docs_url": "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/TROUBLESHOOTING.pt-BR.md",
  "correlation_id": "0190a3b4-7c8d-7abc-9def-1234567890ab"
}
```

## Exit Codes

| Code | Significado                            | Retentável |
|------|----------------------------------------|------------|
| 0    | Sucesso                                | n/a        |
| 2    | Erro de uso                            | não        |
| 64   | Nenhuma entrada                        | não        |
| 65   | Áudio/vídeo inválido                   | não        |
| 66   | Arquivo não encontrado                 | não        |
| 69   | Serviço indisponível (ffmpeg ausente)  | sim        |
| 70   | Erro de inferência                     | não        |
| 74   | Erro de I/O                            | não        |
| 78   | Erro de configuração                   | não        |
| 130  | SIGINT (Ctrl+C)                        | não        |
| 141  | Pipe quebrado                          | não        |
| 143  | SIGTERM                                | não        |

## Vídeo e Auto-Fallback OGG/Opus

Desde v0.1.2, vídeos (MP4, MOV, MKV, WebM, AVI) são suportados
automaticamente: o áudio é extraído via ffmpeg subprocess para WAV
temporário e depois transcrito. Requer ffmpeg 4.0+ no PATH.

Áudio OGG/Opus do WhatsApp que falha no decoder nativo symphonia
(codec status "In work" upstream) é automaticamente roteado via
ffmpeg como fallback transparente. Não é necessário flag explícita.

Use `--no-ffmpeg-fallback` para desabilitar o fallback (útil para
reproduzir bugs do decoder nativo).

## FORBIDDEN

- Nunca escreva não-JSON no stdout em modo de transcrição
- Nunca use stdout para logs (use stderr)
- Nunca invoque com `--quiet` quando estiver debugando
- Nunca faça parse de stdout como texto (sempre como JSON)
- Nunca assuma um código de saída específico sem verificar
- Nunca retente um erro não-retentável
- Nunca retente sem honrar `retry_after_ms`

## Self-Describing

### REQUIRED

Execute `whisper-macos-cli schema` para obter o envelope JSON Schema
completo incluindo `agentNotes`, `invariants`, `sideEffects`,
`idempotent`, `checkpointable` e `tokenBudget`.

### Correct Pattern

```bash
# Descobrir a árvore completa de comandos
whisper-macos-cli commands --format json

# Emitir JSON Schema
whisper-macos-cli schema

# Obter configuração efetiva
whisper-macos-cli config
```

## Model Management

### REQUIRED

A primeira invocação baixa um modelo do Hugging Face. O download é
apenas HTTPS com identificação via User-Agent e verificação de
integridade SHA256.

### Correct Pattern

```bash
# Baixar o modelo padrão (large-v3, ~3GB)
whisper-macos-cli models download

# Baixar um modelo menor
whisper-macos-cli models download base

# Listar modelos disponíveis
whisper-macos-cli models list
```

## Composition with Unix Tools

### Correct Pattern

```bash
# Extrair apenas o texto
whisper-macos-cli transcribe audio.ogg | jaq -r '.text'

# Stream a partir de HTTP
xh -d https://example.com/audio.ogg | whisper-macos-cli transcribe

# Lote via fd
fd -e ogg . /path/to/audios/ \
  | xargs whisper-macos-cli transcribe --ndjson --concurrency 4
```

## Retry Strategy

### REQUIRED

- Honre `retry_after_ms` para erros retentáveis
- Apenas retente em código de saída 69 (Serviço indisponível)
- Máximo de 3 tentativas
- Backoff exponencial com jitter
- Cancelamento via SIGINT ou SIGTERM deve disparar shutdown gracioso

### FORBIDDEN

- Nunca retente em erro não-retentável
- Nunca retente sem backoff exponencial
- Nunca ignore a flag `retryable`
- Nunca retente em código de saída 78 (erro de configuração)

## Environment Variables

- `WHISPER_MODEL` — sobrescreve modelo padrão
- `WHISPER_LANGUAGE` — sobrescreve idioma padrão
- `NO_COLOR` — desabilita saída colorida
- `CI` — desabilita prompts interativos quando 1/true/yes
- `RUST_LOG` — sobrescreve filtro de nível de log
- `SOURCE_DATE_EPOCH` — timestamp Unix para builds reproduzíveis
- `NO_INPUT` — sobrescreve flag --no-input
- `QUIET` — sobrescreve flag --quiet
