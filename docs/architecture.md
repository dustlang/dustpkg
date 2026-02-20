# Architecture

## Workspace Layout

- `Cargo.toml`: workspace root for `crates/dustpkg`.
- `crates/dustpkg/src/main.rs`: CLI argument parsing and command dispatch.
- `crates/dustpkg/src/lib.rs`: manifest model, lockfile model, resolver, and package operations.
- `crates/dustpkg/tests/cli.rs`: integration tests for user-facing CLI behavior.

## Component Roles

## CLI Layer (`main.rs`)

- Parses subcommands: `init`, `add`, `update`, `build`.
- Resolves current directory and `Dust.toml` path.
- Enforces that `Dust.toml` exists for non-`init` commands.
- Delegates all package logic to library functions.

## Library Layer (`lib.rs`)

- Defines data models:
  - `PackageInfo`
  - `Manifest`
  - `LockedDep`
  - `Lockfile`
- Implements I/O:
  - `Manifest::load/save`
  - `Lockfile::load/save`
- Implements operations:
  - `init_package`
  - `add_dependency`
  - `add_stdlib_dependencies`
  - `update_lock`
  - `build_package`
- Implements deterministic resolver:
  - `resolve(manifest, seed)`

## Data Flow

1. Command enters CLI (`main.rs`).
2. CLI validates preconditions (for example `Dust.toml` presence).
3. Library reads/updates manifest.
4. Library resolves dependencies into lock entries.
5. Lockfile is written to `dustpkg.lock`.
6. `build` verifies manifest dependency set matches lock versions.

## Determinism Model

- With `seed = None`: dependencies are sorted alphabetically.
- With `seed = Some(n)`: dependencies are shuffled using `ChaCha8Rng` seeded with `n`.
- Checksums are deterministic SHA-256 over `"name@version"`.
