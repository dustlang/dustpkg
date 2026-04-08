# dustpkg Documentation

This directory contains Markdown documentation for `dustpkg`.

## Documentation Index

- `getting_started.md`: installation, build, and first package workflow.
- `architecture.md`: workspace and execution flow.
- `cli_reference.md`: command-line contract and command behavior.
- `manifest_reference.md`: `Dust.toml` schema and field semantics.
- `lockfile_reference.md`: `dustpkg.lock` schema and deterministic ordering model.
- `resolver_and_reproducibility.md`: resolver algorithm and reproducibility guarantees.
- `library_api.md`: legacy API notes from pre-migration Rust implementation.
- `error_handling.md`: common failures, messages, and operational guidance.
- `testing_and_ci.md`: test strategy and CI workflow.

## Scope

`dustpkg` now ships as a Dust-native top-level grammar profile (`src/main.ds`).
Historical docs that reference the retired Rust crate layout are retained for roadmap context.
