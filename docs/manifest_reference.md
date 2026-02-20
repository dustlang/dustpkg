# Manifest Reference (`Dust.toml`)

## Overview

`Dust.toml` is the project manifest consumed by `dustpkg`.

Current model:

- One package metadata block.
- A flat dependency map of direct dependencies.

## Schema

```toml
[package]
name = "string"
version = "string"
dpl_version = "string" # default: "0.2"

[dependencies]
<dep_name> = "<version>"
```

## Field Semantics

## `[package]`

- `name`: package identifier. `init` defaults this to current directory name.
- `version`: package version string. `init` defaults to `0.1.0`.
- `dpl_version`: Dust language profile version. Defaults to `0.2` when omitted.

## `[dependencies]`

- Keys are dependency names.
- Values are version strings.
- The map is loaded into `HashMap<String, String>`.

## Notes

- `dustpkg` currently treats dependency versions as exact strings.
- There is no advanced constraint solving yet.
- `add` overwrites an existing dependency entry with the new version value.

## Example

```toml
[package]
name = "xdv-shell"
version = "0.2.0"
dpl_version = "0.2"

[dependencies]
xdv-runtime = "0.2.0"
xdv-xdvfs = "0.2.0"
```
