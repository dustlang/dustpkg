# Getting Started

## Prerequisites

- Rust toolchain compatible with edition 2021.
- `cargo` available in PATH.
- A working directory where `dustpkg` can create `Dust.toml` and `dustpkg.lock`.

## Build

From `dustpkg/`:

```bash
cargo build --release
```

Run the CLI with:

```bash
cargo run -p dustpkg -- <command>
```

## First Project Workflow

Create a new project directory and initialize:

```bash
mkdir hello_dust
cd hello_dust
cargo run -p dustpkg -- init
```

Expected result:

- `Dust.toml` is created.
- Package name defaults to the current directory name.
- Version defaults to `0.1.0`.
- `dpl_version` defaults to `0.2`.

Add dependencies:

```bash
cargo run -p dustpkg -- add serde 1.0.0
cargo run -p dustpkg -- add rand 0.8.5 --seed 42
```

Expected result:

- `Dust.toml` dependency map is updated.
- `dustpkg.lock` is regenerated.
- Checksums are recorded for each dependency.

Update lock file without changing manifest:

```bash
cargo run -p dustpkg -- update
```

Validate manifest-lock consistency:

```bash
cargo run -p dustpkg -- build
```

## Typical Files After Setup

`Dust.toml`:

```toml
[package]
name = "hello_dust"
version = "0.1.0"
dpl_version = "0.2"

[dependencies]
serde = "1.0.0"
rand = "0.8.5"
```

`dustpkg.lock` includes:

- `package` metadata copied from manifest.
- `dependencies` array with `name`, `version`, `checksum`, and `source`.
- Optional `seed` when provided.
