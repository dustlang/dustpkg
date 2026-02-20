# Testing and CI

## Test Coverage

Integration tests live in `crates/dustpkg/tests/cli.rs`.

Current tests validate:

- `init` creates `Dust.toml`.
- `add` updates manifest and creates `dustpkg.lock` with checksum fields.
- `build` succeeds after dependency add.
- `update --seed` can produce lockfiles with different ordering for different seeds.

## Local Test Commands

From `dustpkg/`:

```bash
cargo test --workspace --verbose
```

Build-only check:

```bash
cargo build --workspace --verbose
```

## CI Workflow

GitHub Actions workflow: `.github/workflows/ci.yml`.

Pipeline stages:

1. Checkout repository.
2. Install stable Rust toolchain.
3. `cargo build --workspace --verbose`
4. `cargo test --workspace --verbose`

## Adding New Tests

When extending `dustpkg`, prefer integration tests that exercise the CLI end-to-end:

- create temp workspace
- run CLI command sequence
- assert file content and error behavior

This keeps command contracts and on-disk formats stable over time.
