[English version](docs/MIGRATION.md) | [Versão em Português Brasileiro](docs/MIGRATION.pt-BR.md)

# Migration Guide

## What Changes

This is the first public release of whisper-macos-cli. The
`schema_version` is `0.1.0`. Future versions will document any
breaking changes here.

## Step-by-Step Migration

There is no prior version to migrate from. Future 0.x versions
will document incremental migration steps here.

## JSON Schema Changes

The envelope schema is stable within a MAJOR version. Breaking
schema changes require a MAJOR version bump and a `schema_version`
update.

| Field                | Type    | Required | Since |
|----------------------|---------|----------|-------|
| schema_version       | string  | yes      | 0.1.0 |
| correlation_id       | string  | yes      | 0.1.0 |
| file                 | string  | yes      | 0.1.0 |
| language             | string  | yes      | 0.1.0 |
| language_source      | string  | yes      | 0.1.0 |
| model                | string  | yes      | 0.1.0 |
| duration_seconds     | number  | yes      | 0.1.0 |
| text                 | string  | yes      | 0.1.0 |
| segments             | array   | no       | 0.1.0 |
| vad_chunks           | integer | yes      | 0.1.0 |
| processing_time_ms   | integer | yes      | 0.1.0 |

## Compatibility Notes

The CLI does NOT maintain backward compatibility for flags marked
as internal. Public flags follow SemVer.

## Rollback

To roll back to a previous version:

```bash
cargo install whisper-macos-cli --version 0.1.0 --force
```

## See Also

- [CHANGELOG.md](../CHANGELOG.md) — Full release history
- [docs/TESTING.md](docs/TESTING.md) — Test execution guide
- [CONTRIBUTING.md](../CONTRIBUTING.md) — How to contribute
