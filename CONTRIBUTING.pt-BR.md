[English version](CONTRIBUTING.md) | [Versão em Português Brasileiro](CONTRIBUTING.pt-BR.md)

# Contribuindo com whisper-macos-cli

## Bem-vindo

Obrigado pelo seu interesse em contribuir. Este projeto visa ser a CLI
de transcrição de áudio local mais confiável para agentes de IA em
macOS Apple Silicon.

## Início Rápido

- Instale Rust 1.88+ via [rustup](https://rustup.rs)
- Instale cmake: `brew install cmake`
- Instale Xcode CLI Tools: `xcode-select --install`
- Faça fork e clone do repositório
- Execute `cargo build` para verificar o build

## Configuração de Desenvolvimento

- Execute `cargo test` para rodar testes unitários e de integração
- Execute `cargo fmt --check` para verificar formatação
- Execute `cargo clippy --all-targets -- -D warnings` para verificar lints
- Execute `cargo audit` para verificar vulnerabilidades conhecidas
- Execute `cargo deny check` para verificar licenças e crates banidos

## Estratégia de Branching

- Crie branch a partir de `main`
- Use kebab-case: `fix-sigpipe-handler`, `add-bonjour`
- Uma preocupação por branch
- Rebase sobre `main` antes de abrir pull request

## Convenção de Commits

Este projeto usa [Conventional Commits](https://www.conventionalcommits.org/pt-br/).

- `feat` — nova feature
- `fix` — correção de bug
- `docs` — apenas documentação
- `chore` — tooling ou mudança não-funcional
- `refactor` — mudança de código que não corrige bug nem adiciona feature
- `test` — adições ou modificações em testes
- `perf` — melhoria de performance
- `ci` — configuração de CI

## Processo de PR

1. Abra pull request apontando para `main`
2. Preencha o template de pull request
3. Passe em todas as verificações de CI
4. Receba aprovação de pelo menos um mantenedor
5. Squash and merge

## Testes

- Testes unitários ficam próximos ao código em `#[cfg(test)] mod tests`
- Testes de integração ficam em `tests/`
- Testes de propriedade usam `proptest` e rodam em `cargo test`
- Testes de snapshot usam `insta` e requerem `cargo insta review`
- Testes de fuzz usam `cargo-fuzz` e rodam em CI semanalmente

## Documentação

Toda mudança pública deve atualizar:

- `CHANGELOG.md` sob `## [Unreleased]`
- `CHANGELOG.pt-BR.md` sob `## [Não Lançado]` na mesma edição
- A seção relevante em `README.md`
- A seção relevante em `README.pt-BR.md`
- Doc comments em itens públicos alterados

## Reportar Bugs

Abra um bug report em
https://github.com/daniloaguiarbr/whisper-macos-cli/issues/new?template=bug.md

## Solicitar Features

Abra um feature request em
https://github.com/daniloaguiarbr/whisper-macos-cli/issues/new

## Processo de Release

- Mantenedores cortam releases usando `cargo release`
- Cada release bumpa a versão em `Cargo.toml`
- Cada release atualiza `CHANGELOG.md` e `CHANGELOG.pt-BR.md`
- Cada release cria uma git tag
- Cada release dispara o workflow de release
- Cada release publica no crates.io via Trusted Publishing (OIDC)

## Reconhecimento

Contribuidores são listados em
`git log --format='%aN' | sort -u`. Contribuições significativas são
mencionadas nas release notes.

## Perguntas

Abra uma GitHub Discussion em
https://github.com/daniloaguiarbr/whisper-macos-cli/discussions.
