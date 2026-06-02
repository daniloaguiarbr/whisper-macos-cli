[English version](CONTRIBUTING.md) | [Versão em Português Brasileiro](CONTRIBUTING.pt-BR.md)

# Contributing to whisper-macos-cli

## Welcome

Thank you for your interest in contributing. This project aims to be
the most reliable local audio transcription CLI for AI agents on macOS
Apple Silicon.

## Quick Start

- Install Rust 1.88+ via [rustup](https://rustup.rs)
- Install cmake: `brew install cmake`
- Install Xcode CLI Tools: `xcode-select --install`
- Fork and clone the repository
- Run `cargo build` to verify the build

## Development Setup

- Run `cargo test` to execute unit and integration tests
- Run `cargo fmt --check` to verify formatting
- Run `cargo clippy --all-targets -- -D warnings` to verify lints
- Run `cargo audit` to check for known vulnerabilities
- Run `cargo deny check` to check licenses and banned crates

## Branching Strategy

- Branch from `main`
- Use kebab-case branch names: `fix-sigpipe-handler`, `add-bonjour`
- One concern per branch
- Rebase onto `main` before opening a pull request

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/).

- `feat` — new feature
- `fix` — bug fix
- `docs` — documentation only
- `chore` — tooling or non-functional change
- `refactor` — code change that neither fixes a bug nor adds a feature
- `test` — test additions or modifications
- `perf` — performance improvement
- `ci` — CI configuration

## PR Process

1. Open a pull request targeting `main`
2. Fill in the pull request template
3. Pass all CI checks
4. Receive approval from at least one maintainer
5. Squash and merge

## Testing

- Unit tests live next to the code they test in `#[cfg(test)] mod tests`
- Integration tests live in `tests/`
- Property-based tests use `proptest` and run on `cargo test`
- Snapshot tests use `insta` and require `cargo insta review`
- Fuzz tests use `cargo-fuzz` and run in CI on a weekly schedule

## Documentation

Every public change must update:

- `CHANGELOG.md` under `## [Unreleased]`
- `CHANGELOG.pt-BR.md` under `## [Não Lançado]` in the same edit
- The relevant section in `README.md`
- The relevant section in `README.pt-BR.md`
- Doc comments on changed public items

## Report Bugs

Open a bug report at
https://github.com/daniloaguiarbr/whisper-macos-cli/issues/new?template=bug.md

## Request Features

Open a feature request at
https://github.com/daniloaguiarbr/whisper-macos-cli/issues/new

## Release Process

- Maintainers cut releases using `cargo release`
- Each release bumps the version in `Cargo.toml`
- Each release updates `CHANGELOG.md` and `CHANGELOG.pt-BR.md`
- Each release creates a git tag
- Each release triggers the release workflow
- Each release publishes to crates.io via Trusted Publishing (OIDC)

## Recognition

Contributors are listed in
`git log --format='%aN' | sort -u`. Significant contributions are
mentioned in release notes.

## Questions

Open a GitHub Discussion at
https://github.com/daniloaguiarbr/whisper-macos-cli/discussions.
