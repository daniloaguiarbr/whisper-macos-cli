[English version](docs/TESTING.md) | [Versão em Português Brasileiro](docs/TESTING.pt-BR.md)

# Guia de Testes

## Por Que Testes Categorizados

A suíte de testes é dividida em categorias que mapeiam para estágios
de CI. Uma categoria rápida roda em todo commit; uma categoria lenta
roda noturnamente.

## Categorias de Testes

- Testes unitários: `cargo test --lib`
- Testes de integração: `cargo test --test cli`
- Testes de documentação: `cargo test --doc`
- Testes de propriedade: incluídos em unit tests via `proptest`
- Testes de snapshot: `cargo insta test` e `cargo insta review`
- Testes de fuzz: `cargo +nightly fuzz run <target>` (workflow separado)

## Como Rodar

### Rodar todos os testes localmente

```bash
cargo test
```

### Rodar apenas testes unitários

```bash
cargo test --lib
```

### Rodar apenas testes de integração

```bash
cargo test --test cli
```

### Rodar um único teste por nome

```bash
cargo test nome_do_teste
```

### Rodar com todas as features

```bash
cargo test --all-features
```

### Rodar sem features padrão

```bash
cargo test --no-default-features
```

## Perfis de CI

- Pull request: fmt, clippy, test (lib + integration + doc)
- Noturno: test (all-features), audit, deny, coverage, semver-checks
- Semanal: fuzz, mutants, miri
- Release: publish dry-run, doc, build (all targets)

## Variáveis de Ambiente

- `RUST_LOG` — define nível de log de tracing
- `INSTA_UPDATE` — defina como `no` em CI para falhar em novos snapshots
- `RUSTFLAGS` — passa para todas as invocações `cargo`
- `CARGO_TERM_COLOR` — defina como `always` para saída colorida

## Solução de Problemas

### Teste trava

Provavelmente um teste que requer acesso à rede. Defina `CI=true`
para pular ou rode o teste isoladamente com
`cargo test --test cli nome`.

### Teste falha em macOS

Verifique a versão do Xcode CLI Tools com `xcode-select -p` e a
versão do cmake com `cmake --version`.

### Teste de snapshot falha

Rode `cargo insta review` para inspecionar o diff e aceite
(`cargo insta accept`) ou atualize o snapshot no código.
