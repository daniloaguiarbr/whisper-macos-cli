[English version](MIGRATION.md) | [Versão em Português Brasileiro](MIGRATION.pt-BR.md)

# Guia de Migração

## 0.1.0 / 0.1.1 → 0.1.2

### O Que Muda

A versão 0.1.2 introduz suporte a containers de vídeo e corrige o
bug de decode OGG/Opus para mensagens de voz do WhatsApp. O campo
`schema_version` de cada envelope de saída permanece `0.1.0` para
compatibilidade retroativa. As URLs `$id` dos schemas refletem a
superfície mais recente.

### Novas Variantes de Erro

Três novas variantes foram adicionadas ao enum `Error`. Agentes
existentes que tratam erros pelo campo `code` continuam funcionando
sem mudanças.

| Variante                 | Código de saída | Categoria | Retentável |
|--------------------------|-----------------|-----------|------------|
| `VideoExtractionFailed`  | 65              | data      | não        |
| `FfmpegNotFound`         | 69              | service   | não        |
| `UnsupportedVideoFormat` | 65              | data      | não        |

### Novas Flags da CLI

| Flag                       | Env var                       | Padrão   | Desde |
|----------------------------|-------------------------------|----------|-------|
| `--ffmpeg-binary <PATH>`   | `WHISPER_FFMPEG_BINARY`       | `ffmpeg` | 0.1.2 |
| `--no-ffmpeg-fallback`     | `WHISPER_NO_FFMPEG_FALLBACK`  | `false`  | 0.1.2 |

A flag `--no-ffmpeg-fallback` desabilita o fallback transparente
de OGG/Opus para ffmpeg. Use para reproduzir bugs do decoder nativo.

### Novo Módulo: `src/video/`

O novo módulo `video` contém detecção de magic bytes e trait
`FfmpegRunner`. Consumidores da API pública em Rust não precisam
importar nada deste módulo. A CLI cuida do roteamento de forma
transparente.

### Nova Assinatura em `src/audio/decode.rs`

A função interna `decode_file` agora aceita um `FfmpegRunner`
opcional e flag `auto_fallback`. A API pública em Rust não muda.
A CLI usa o runner padrão `RealFfmpeg`.

### Migração Passo a Passo

1. Atualize a dependência: `cargo update -p whisper-macos-cli`
2. Verifique se ffmpeg está instalado: `ffmpeg -version`
3. Se ffmpeg não estiver instalado: `brew install ffmpeg`
4. Pipelines existentes continuam funcionando sem mudanças
5. Opcional: passe `--no-ffmpeg-fallback` para desabilitar fallback

### Mudanças Quebrantes

Nenhuma. v0.1.2 é totalmente retrocompatível com v0.1.0 e v0.1.1.

## 0.1.0 → 0.1.1

### O Que Muda

A versão 0.1.1 só alterou a licença de MIT-only para dual
MIT OR Apache-2.0. Nenhuma mudança de código ou contrato.

## 0.0.x → 0.1.0

### O Que Muda

Este é o primeiro lançamento público. Não há versão anterior
para migrar.

## Mudanças no JSON Schema

O esquema do envelope é estável dentro de uma versão MAJOR.
Mudanças quebrantes de esquema requerem um bump MAJOR e uma
atualização do `schema_version`.

| Campo                | Tipo    | Obrigatório | Desde |
|----------------------|---------|-------------|-------|
| schema_version       | string  | sim         | 0.1.0 |
| correlation_id       | string  | sim         | 0.1.0 |
| file                 | string  | sim         | 0.1.0 |
| language             | string  | sim         | 0.1.0 |
| language_source      | string  | sim         | 0.1.0 |
| model                | string  | sim         | 0.1.0 |
| duration_seconds     | number  | sim         | 0.1.0 |
| text                 | string  | sim         | 0.1.0 |
| segments             | array   | não         | 0.1.0 |
| vad_chunks           | integer | sim         | 0.1.0 |
| processing_time_ms   | integer | sim         | 0.1.0 |

Novos campos em `transcribe-input` adicionados em 0.1.2:

| Campo                  | Tipo    | Obrigatório | Desde |
|------------------------|---------|-------------|-------|
| ffmpeg_binary          | string  | não         | 0.1.2 |
| no_ffmpeg_fallback     | boolean | não         | 0.1.2 |

## Notas de Compatibilidade

A CLI NÃO mantém compatibilidade retroativa para flags marcadas
como internas. Flags públicas seguem SemVer.

## Rollback

Para voltar a uma versão anterior:

```bash
cargo install whisper-macos-cli --version 0.1.0 --force
```

## Veja Também

- [CHANGELOG.pt-BR.md](../CHANGELOG.pt-BR.md) — Histórico completo
- [docs/TESTING.pt-BR.md](TESTING.pt-BR.md) — Guia de execução de testes
- [CONTRIBUTING.pt-BR.md](../CONTRIBUTING.pt-BR.md) — Como contribuir
