[English version](README.md) | [Versão em Português Brasileiro](README.pt-BR.md)

# Schemas Index

Machine-readable JSON Schema definitions for every command contract of
whisper-macos-cli. Each schema is versioned alongside the
`schema_version` field of the envelope it describes.

## English

| Schema                                  | Command        | Version | Purpose                              |
|-----------------------------------------|----------------|---------|--------------------------------------|
| `transcribe-result.schema.json`         | transcribe     | 0.1.2   | Transcription result envelope        |
| `transcribe-input.schema.json`          | transcribe     | 0.1.2   | CLI flags accepted by transcribe      |
| `models-list.schema.json`               | models list    | 0.1.2   | List of available and cached models  |
| `models-download.schema.json`           | models download| 0.1.2   | Status envelope after download        |
| `models-remove.schema.json`             | models remove  | 0.1.2   | Status envelope after remove          |
| `error.schema.json`                     | any            | 0.1.2   | Error envelope shared across commands|
| `schema-envelope.schema.json`           | schema         | 0.1.2   | Self-describing envelope metadata    |
| `commands-tree.schema.json`             | commands       | 0.1.2   | Full command tree structure          |

## Português Brasileiro

| Schema                                  | Comando         | Versão | Propósito                            |
|-----------------------------------------|-----------------|--------|--------------------------------------|
| `transcribe-result.schema.json`         | transcribe      | 0.1.2  | Envelope de resultado de transcrição |
| `transcribe-input.schema.json`          | transcribe      | 0.1.2  | Flags aceitas pelo transcribe        |
| `models-list.schema.json`               | models list     | 0.1.2  | Lista de modelos disponíveis/cacheados|
| `models-download.schema.json`           | models download | 0.1.2  | Envelope de status após download     |
| `models-remove.schema.json`             | models remove   | 0.1.2  | Envelope de status após remove       |
| `error.schema.json`                     | qualquer        | 0.1.2  | Envelope de erro compartilhado       |
| `schema-envelope.schema.json`           | schema          | 0.1.2  | Metadados do envelope auto-descritivo|
| `commands-tree.schema.json`             | commands        | 0.1.2  | Estrutura completa da árvore        |

## Versioning

Each schema includes a `$id` field that includes the version, such as
`https://github.com/daniloaguiarbr/whisper-macos-cli/schemas/transcribe-result/v0.1.2.json`.
Breaking changes to a schema require a MAJOR version bump and a new
`$id`.

The schema version is decoupled from the CLI version. The CLI
currently emits `schema_version: 0.1.0` in every envelope; the
`$id` reflects the latest schema surface. New CLI versions may
introduce new fields without bumping the schema `$id` if backward
compatibility is preserved.

## Validation

Validate output against a schema using any JSON Schema 2020-12
validator, for example:

```bash
npm install -g ajv-cli
whisper-macos-cli transcribe audio.ogg | ajv validate \
  -s docs/schemas/transcribe-result.schema.json -d /dev/stdin --spec=draft2020
```
