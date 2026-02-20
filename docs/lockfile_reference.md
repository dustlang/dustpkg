# Lockfile Reference (`dustpkg.lock`)

## Overview

`dustpkg.lock` captures a deterministic install plan derived from `Dust.toml`.

It records:

- package metadata
- resolved direct dependencies
- per-dependency checksum
- deterministic seed (optional)

## Schema

```toml
seed = 42 # optional top-level key; present only when provided

[package]
name = "string"
version = "string"
dpl_version = "string"

[[dependencies]]
name = "string"
version = "string"
checksum = "hex_sha256"
source = "registry/<name>-<version>"
```

## Dependency Entry Semantics

- `name`: dependency name from manifest.
- `version`: dependency version string from manifest.
- `checksum`: SHA-256 of `"name@version"`, hex encoded.
- `source`: currently synthetic source path `registry/<name>-<version>`.

## Ordering Rules

- No seed: dependency list sorted alphabetically by dependency name.
- Seeded: dependency list shuffled by `ChaCha8Rng` with the provided `u64` seed.

Different seeds can produce different lockfile orderings.

## Validation in `build`

`build` verifies:

- Every manifest dependency exists in `dustpkg.lock`.
- Locked version equals manifest version.

Current `build` does not yet verify checksums against fetched artifacts.

## Example

```toml
seed = 42

[package]
name = "hello_dust"
version = "0.1.0"
dpl_version = "0.2"

[[dependencies]]
name = "serde"
version = "1.0.0"
checksum = "..."
source = "registry/serde-1.0.0"
```
