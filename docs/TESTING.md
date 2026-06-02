[English version](docs/TESTING.md) | [Versão em Português Brasileiro](docs/TESTING.pt-BR.md)

# Testing Guide

## Why Categorized Tests

The test suite is divided into categories that map to CI stages. A
fast category runs on every commit; a slow category runs nightly.

## Test Categories

- Unit tests: `cargo test --lib`
- Integration tests: `cargo test --test cli`
- Documentation tests: `cargo test --doc`
- Property tests: included in unit tests via `proptest`
- Snapshot tests: `cargo insta test` and `cargo insta review`
- Fuzz tests: `cargo +nightly fuzz run <target>` (separate workflow)

## How to Run

### Run all tests locally

```bash
cargo test
```

### Run only unit tests

```bash
cargo test --lib
```

### Run only integration tests

```bash
cargo test --test cli
```

### Run a single test by name

```bash
cargo test name_of_test
```

### Run with all features

```bash
cargo test --all-features
```

### Run with no default features

```bash
cargo test --no-default-features
```

## CI Profiles

- Pull request: fmt, clippy, test (lib + integration + doc)
- Nightly: test (all-features), audit, deny, coverage, semver-checks
- Weekly: fuzz, mutants, miri
- Release: publish dry-run, doc, build (all targets)

## Environment Variables

- `RUST_LOG` — set tracing log level
- `INSTA_UPDATE` — set to `no` in CI to fail on new snapshots
- `RUSTFLAGS` — pass through to all `cargo` invocations
- `CARGO_TERM_COLOR` — set to `always` for colored output

## Troubleshooting

### Test hangs

Likely a test that requires network access. Set `CI=true` to skip
or run the test in isolation with `cargo test --test cli name`.

### Test fails on macOS

Check the Xcode CLI Tools version with `xcode-select -p` and the
cmake version with `cmake --version`.

### Snapshot test fails

Run `cargo insta review` to inspect the diff and either accept
(`cargo insta accept`) or update the snapshot in the source.
