[English version](docs/MIGRATION.md) | [Versão em Português Brasileiro](docs/MIGRATION.pt-BR.md)

# Guia de Migração

## O Que Muda

Este é o primeiro lançamento público do whisper-macos-cli. O
`schema_version` é `0.1.0`. Versões futuras documentarão quaisquer
mudanças quebrantes aqui.

## Migração Passo a Passo

Não há versão anterior para migrar. Futuras versões 0.x
documentarão passos incrementais de migração aqui.

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
- [docs/TESTING.md](docs/TESTING.md) — Guia de execução de testes
- [CONTRIBUTING.pt-BR.md](../CONTRIBUTING.pt-BR.md) — Como contribuir
