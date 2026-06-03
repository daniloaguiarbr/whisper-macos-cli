[English version](docs/COOKBOOK.md) | [Versão em Português Brasileiro](docs/COOKBOOK.pt-BR.md)

> Vinte receitas prontas para produção cobrindo ingestão, recuperação e auditoria.

# Cookbook

## Nota de Latência

- Cold start com large-v3: 2-5 segundos de warmup
- Transcrições subsequentes: aproximadamente tempo real em M2 Pro
- Concorrência escala linearmente até 8 em M1 Pro

## Referência de Valores Padrão

| Parâmetro        | Padrão     | Override                |
|------------------|------------|-------------------------|
| Modelo           | large-v3   | `--model`, `WHISPER_MODEL` |
| Idioma           | OS locale  | `--language`, `WHISPER_LANGUAGE` |
| Beam size        | 8          | `--beam-size`           |
| Threshold VAD    | 0.5        | `--vad-threshold`       |
| Concorrência     | 2          | `--concurrency`         |
| Formato de saída | JSON       | `--ndjson`              |

## Como Transcrever um Único Arquivo WAV

```bash
whisper-macos-cli transcribe fala.wav
```

```json
{"schema_version":"0.1.0","correlation_id":"...","file":"fala.wav","language":"pt","language_source":"os_locale","model":"large-v3","duration_seconds":12.4,"text":"Olá mundo","vad_chunks":1,"processing_time_ms":1820}
```

## Como Transcrever uma Mensagem de Voz do WhatsApp (OGG/Opus)

```bash
whisper-macos-cli transcribe audio-zap.ogg
```

O pre-skip do Opus (3840 samples a 48kHz) é descartado automaticamente.

## Como Transcrever a partir do stdin

```bash
cat gravacao.ogg | whisper-macos-cli transcribe
```

Stdin é limitado a 2 GB para evitar OOM.

## Como Transcrever em Lote como NDJSON

```bash
whisper-macos-cli transcribe *.ogg --ndjson --concurrency 4
```

Cada arquivo emite um objeto JSON no stdout. Uma linha final
`{"summary": true, ...}` reporta os totais.

## Como Forçar um Idioma

```bash
whisper-macos-cli transcribe --language pt audio.wav
```

## Como Usar um Modelo Específico

```bash
whisper-macos-cli models download small
whisper-macos-cli transcribe --model small audio.wav
```

## Como Definir Beam Size Customizado

```bash
whisper-macos-cli transcribe --beam-size 4 audio.wav
```

Intervalo válido: 1-16. Maior é mais lento mas mais preciso.

## Como Obter Segmentos com Timestamp

```bash
whisper-macos-cli transcribe --timestamps audio.wav
```

Adiciona array `segments` com `start`, `end`, `text` por segmento.

## Como Desabilitar VAD

```bash
whisper-macos-cli transcribe --vad-threshold 0.0 audio.wav
```

Threshold 0.0 efetivamente desabilita VAD, transcrevendo o áudio
inteiro sem segmentação de fala.

## Como Rodar com Verbosidade Máxima

```bash
whisper-macos-cli -vvv transcribe audio.wav
```

## Como Rodar em Silêncio

```bash
whisper-macos-cli --quiet transcribe audio.wav
```

Suprime toda saída stderr.

## Como Listar Modelos Disponíveis

```bash
whisper-macos-cli models list
```

## Como Baixar o Modelo Padrão

```bash
whisper-macos-cli models download
```

Baixa `large-v3` (~3 GB).

## Como Baixar um Modelo Específico

```bash
whisper-macos-cli models download small
```

## Como Imprimir o Caminho do Arquivo de Modelo

```bash
whisper-macos-cli models path small
```

## Como Remover um Modelo (dry run)

```bash
whisper-macos-cli models remove tiny --dry-run
```

## Como Remover um Modelo (real)

```bash
whisper-macos-cli models remove tiny
```

## Como Diagnosticar o Ambiente

```bash
whisper-macos-cli doctor
```

Retorna código de saída 0 se todas as verificações passam, 78 caso
contrário.

## Como Obter o JSON Schema

```bash
whisper-macos-cli schema
```

Retorna o esquema completo do envelope incluindo agentNotes,
invariants, sideEffects, idempotent, checkpointable e tokenBudget.

## Como Obter a Configuração Efetiva

```bash
whisper-macos-cli config
```

Retorna a configuração efetiva atual como JSON.

## Como Validar Entradas Sem Transcrever

```bash
whisper-macos-cli transcribe --dry-run audio.ogg
```

Resolve entradas, modelo e idioma sem carregar o modelo ou rodar
inferência.

## Como Usar em CI/CD

```bash
CI=true whisper-macos-cli transcribe --quiet --no-input \
  --language en audio.ogg > resultado.json
```

`CI=true` desabilita prompts interativos. `--no-input` é honrado
automaticamente. `--quiet` suprime stderr.

## Como Transcrever um Arquivo de Vídeo

```bash
whisper-macos-cli transcribe video.mp4
```

Requer ffmpeg 4.0+ no PATH. Veja
[VIDEO-EXTRACTION.pt-BR.md](VIDEO-EXTRACTION.pt-BR.md) para a
lista completa de containers suportados e garantias de
segurança.

## Como Transcrever em Lote uma Pasta de Vídeos

```bash
whisper-macos-cli transcribe --ndjson --concurrency 2 *.mp4
```

Concorrência é limitada em fluxos de vídeo porque cada
transcrição faz spawn de um subprocesso ffmpeg. Em máquinas de
4 cores, `--concurrency 2` é seguro.

## Como Usar um Binário ffmpeg Customizado

```bash
whisper-macos-cli transcribe --ffmpeg-binary /opt/local/bin/ffmpeg video.mov
```

Use quando o ffmpeg está instalado via MacPorts, Homebrew ou
prefixo customizado e não está no PATH padrão.

## Como Reproduzir um Bug do Decoder Nativo

```bash
whisper-macos-cli transcribe --no-ffmpeg-fallback audio.ogg
```

Desabilita o fallback transparente para ffmpeg. Use ao debugar
o comportamento do decoder nativo do symphonia.

## Como Verificar que o Fallback do ffmpeg Está Acionando

```bash
whisper-macos-cli transcribe -v audio.ogg
# stderr: ... native decode failed, attempting ffmpeg fallback
```

A flag `-v` aumenta a verbosidade para que a decisão de fallback
seja logada no stderr.
