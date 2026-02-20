# dustpkg Documentation

This directory contains complete Markdown documentation for `dustpkg`.

## Documentation Index

- `getting_started.md`: installation, build, and first package workflow.
- `architecture.md`: workspace, crate, and execution flow.
- `cli_reference.md`: command-line contract and command behavior.
- `manifest_reference.md`: `Dust.toml` schema and field semantics.
- `lockfile_reference.md`: `dustpkg.lock` schema and deterministic ordering model.
- `resolver_and_reproducibility.md`: resolver algorithm and reproducibility guarantees.
- `library_api.md`: public Rust API surface in `crates/dustpkg/src/lib.rs`.
- `error_handling.md`: common failures, messages, and operational guidance.
- `testing_and_ci.md`: test coverage and CI workflow.

## Scope

`dustpkg` is a deterministic package manager prototype for Dust projects. It initializes package manifests, manages direct dependencies, generates lock files with checksums, and validates manifest-lock consistency during build.

Current behavior focuses on deterministic dependency resolution and lockfile generation. It does not yet fetch remote registries or compile package artifacts.
