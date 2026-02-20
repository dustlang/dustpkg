# Library API (`crates/dustpkg/src/lib.rs`)

## Core Data Types

## `PackageInfo`

```rust
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub dpl_version: String,
}
```

Represents `[package]` metadata.

## `Manifest`

```rust
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: HashMap<String, String>,
}
```

Methods:

- `Manifest::load(path: &Path) -> Result<Manifest>`
- `Manifest::save(&self, path: &Path) -> Result<()>`

## `LockedDep`

```rust
pub struct LockedDep {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub source: String,
}
```

Represents one resolved dependency in lockfile.

## `Lockfile`

```rust
pub struct Lockfile {
    pub package: PackageInfo,
    pub dependencies: Vec<LockedDep>,
    pub seed: Option<u64>,
}
```

Methods:

- `Lockfile::load(path: &Path) -> Result<Lockfile>`
- `Lockfile::save(&self, path: &Path) -> Result<()>`

## Operational Functions

- `resolve(manifest: &Manifest, seed: Option<u64>) -> Lockfile`
- `init_package(dir: &Path) -> Result<()>`
- `add_dependency(manifest_path: &Path, dep_name: &str, dep_version: &str, seed: Option<u64>) -> Result<()>`
- `add_stdlib_dependencies(manifest_path: &Path, seed: Option<u64>) -> Result<()>`
- `update_lock(manifest_path: &Path, seed: Option<u64>) -> Result<()>`
- `build_package(manifest_path: &Path, seed: Option<u64>) -> Result<()>`

## Notes on `add_stdlib_dependencies`

`add_stdlib_dependencies` is currently a library-level function and not wired to a dedicated CLI command.

Behavior:

- Always adds `dustlib = "0.2.0"`.
- Adds `dustlib_k = "0.2.0"` when `dpl_version == "0.2"`.
- Saves manifest and regenerates lockfile.

## API Stability

`dustpkg` is currently version `0.2.0` and still prototype-stage. Public APIs may evolve as dependency resolution and build integration expand.
