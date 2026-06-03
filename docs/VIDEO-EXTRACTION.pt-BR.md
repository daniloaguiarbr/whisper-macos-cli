# Extração de Vídeo — whisper-macos-cli v0.1.2+

Desde a v0.1.2, o whisper-macos-cli consegue transcrever áudio a
partir de containers de vídeo. A trilha de áudio do vídeo é extraída
para um WAV temporário via ffmpeg, e depois alimentada para o pipeline
regular do whisper.cpp.

## Formatos Suportados

| Container | Magic bytes           | Notas                          |
|-----------|-----------------------|--------------------------------|
| MP4       | `....ftypisom`        | Mais comum; exports do YouTube |
| MOV       | `....ftypqt  `        | Apple QuickTime                |
| M4V       | `....ftypM4V `        | Vídeo do iTunes                |
| MKV       | `0x1A 0x45 0xDF 0xA3` | Matroska                       |
| WebM      | `0x1A 0x45 0xDF 0xA3` | Derivado de Matroska           |
| AVI       | `RIFF....AVI `        | Windows legado                 |
| M4A       | `....ftypM4A `        | Áudio MPEG-4 (frequente)       |
| FLV       | `FLV\x01`             | Flash Video                    |
| WMV/WMA   | `0x30 0x26 0xB2 0x75` | Container ASF                  |

A detecção usa magic bytes primeiro, depois a extensão do arquivo.
Arquivos renomeados (`.ogg` com magic MP4) são roteados corretamente.

## Requisitos

- **ffmpeg 4.0 ou superior** deve estar instalado e acessível no
  `PATH`, ou sua localização deve ser especificada via
  `--ffmpeg-binary` ou a variável de ambiente `WHISPER_FFMPEG_BINARY`.

### Instalar o ffmpeg

- **macOS:** `brew install ffmpeg`
- **Ubuntu/Debian:** `sudo apt-get install ffmpeg`
- **Windows (Chocolatey):** `choco install ffmpeg`
- **Windows (winget):** `winget install Gyan.FFmpeg`

## Uso

### Transcrever um arquivo de vídeo

```bash
whisper-macos-cli transcribe video.mp4
```

A saída é um único envelope JSON (ou linha NDJSON em modo batch)
idêntico ao de uma transcrição de áudio regular. O campo `file`
contém o nome do arquivo de vídeo.

### Transcrição em lote de uma pasta de vídeos

```bash
whisper-macos-cli transcribe --ndjson --concurrency 4 *.mp4
```

### Especificar binário ffmpeg customizado

```bash
whisper-macos-cli transcribe --ffmpeg-binary /opt/local/bin/ffmpeg video.mov
```

Ou via variável de ambiente:

```bash
export WHISPER_FFMPEG_BINARY=/opt/local/bin/ffmpeg
whisper-macos-cli transcribe video.mkv
```

### Desabilitar fallback ffmpeg completamente

Se você quiser testar que o decoder nativo do symphonia é suficiente
(reproduzindo o bug do OGG/Opus, por exemplo), pode desabilitar o
fallback:

```bash
whisper-macos-cli transcribe --no-ffmpeg-fallback audio.ogg
```

Quando esta flag é setada e um arquivo de vídeo é fornecido, a CLI
retorna `Error::UnsupportedVideoFormat` (exit 65) em vez de tentar
a extração.

## Auto-Fallback OGG/Opus

Os arquivos OGG/Opus produzidos pelo WhatsApp (e outros mensageiros
de voz) acionam um bug conhecido no crate `symphonia`
([Issue #8](https://github.com/pdeljanov/Symphonia/issues/8)) — o
status "Opus" é oficialmente listado como **"In work"** pelo
projeto. A partir da v0.1.2, o whisper-macos-cli detecta
transparentemente essa falha e re-executa o decode via ffmpeg, que
lida com o codec corretamente. O fallback é automático e produz
saída idêntica a um decode nativo bem-sucedido.

Para verificar que o fallback está acontecendo, rode com `-v`:

```bash
whisper-macos-cli transcribe -v audio.ogg
# stderr: ... native decode failed, attempting ffmpeg fallback
```

## ffmpeg Não Encontrado

Se o ffmpeg não está instalado e o input é vídeo (ou o decode nativo
falha), a CLI retorna:

```json
{
  "schema_version": "0.1.2",
  "error": true,
  "code": 69,
  "message": "ffmpeg not found in PATH: install via `brew install ffmpeg` or set --ffmpeg-binary",
  "category": "service",
  "retryable": false,
  "retry_after_ms": null,
  "hint": "install ffmpeg via `brew install ffmpeg` or set --ffmpeg-binary",
  "docs_url": "https://github.com/daniloaguiarbr/whisper-macos-cli/blob/main/docs/VIDEO-EXTRACTION.pt-BR.md#ffmpeg-nao-encontrado",
  "correlation_id": "..."
}
```

Código de saída 69. Corrija instalando o ffmpeg (veja acima) e
tentando novamente.

## Segurança e Isolamento de Processo

O subprocesso do ffmpeg é endurecido com as seguintes garantias:

- **`env_clear()`** — nenhuma variável de ambiente do host é
  herdada exceto uma allowlist mínima (`PATH`, `HOME`, `TMPDIR`,
  `LANG`, `LC_ALL`). Secrets como `*_TOKEN` não podem vazar para
  logs do ffmpeg.
- **`setsid()` em Unix / `CREATE_NEW_PROCESS_GROUP` em Windows** —
  o filho roda em seu próprio grupo de processos. Ctrl+C entregue
  ao pai não propaga silenciosamente para o ffmpeg.
- **Kill-on-drop** — o handle do filho é envolvido em um guard
  `SafeChild`. Se o pai entrar em panic, o filho é morto (SIGKILL
  em Unix, TerminateProcess em Windows) para prevenir processos
  zumbi.
- **Timeout limitado** — padrão 180s. No timeout, o filho é morto
  e `Error::VideoExtractionFailed` é retornado.
- **Validação de saída** — o WAV extraído é validado pós-processo:
  deve ter header `RIFF...WAVE`, tamanho deve bater com o chunk
  size do RIFF. Captura a classe de bug "ffmpeg exit 0 mas arquivo
  vazio".
- **Cleanup de temp** — o WAV temporário é removido via guard
  `Drop` mesmo se o decode entrar em panic.

## Limites

- **Duração máxima:** 24 horas (herdado do pipeline de áudio).
- **Tamanho máximo de arquivo:** limitado pelo diretório temp;
  ~3 GB é o teto prático para 1h de vídeo típico.
- **Concorrência:** a flag `--concurrency N` governa quantas
  transcrições rodam em paralelo, cada uma podendo fazer spawn de
  um subprocesso ffmpeg. Em máquinas de 4 cores, `--concurrency 2`
  é seguro.

## Códigos de Saída

| Código | Significado                                       |
|--------|---------------------------------------------------|
| 0      | Sucesso                                           |
| 2      | Erro de uso (argumentos inválidos)                |
| 64     | Nenhuma entrada fornecida                         |
| 65     | Dados inválidos (áudio corrompido, falha de extração de vídeo, formato não suportado) |
| 66     | Arquivo de entrada não encontrado                 |
| 69     | Serviço indisponível (ffmpeg ausente, download do modelo falhou) |
| 70     | Erro de inferência                                |
| 74     | Erro de I/O                                       |
| 78     | Erro de configuração                              |

## Veja Também

- [TROUBLESHOOTING.pt-BR.md](TROUBLESHOOTING.pt-BR.md) — diagnóstico geral
- [SKILL.pt-BR.md](../SKILL.pt-BR.md) — referência do contrato JSON
- [AGENTS.pt-BR.md](../AGENTS.pt-BR.md) — guia de integração com agentes
- [CHANGELOG.pt-BR.md](../CHANGELOG.pt-BR.md) — notas de release
