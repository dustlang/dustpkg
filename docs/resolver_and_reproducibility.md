# Resolver and Reproducibility

## Resolver Algorithm

`resolve(manifest, seed)` performs the following:

1. Copies manifest dependencies into a list of `(name, version)` tuples.
2. Orders the list:
   - alphabetically by name when `seed` is absent
   - deterministic pseudo-random order when `seed` is present
3. Converts each tuple to `LockedDep`:
   - checksum = SHA-256(`"name@version"`)
   - source = `registry/<name>-<version>`
4. Returns `Lockfile` with package metadata and optional `seed`.

## Determinism Guarantees

Given the same:

- `Dust.toml` content
- seed value
- `dustpkg` version

the resolver produces the same lockfile content.

## What Is Reproducible Today

- Manifest-to-lock transform is deterministic.
- Lockfile encodes exact direct dependency versions and checksums.
- Build step checks manifest/lock version consistency.

## What Is Not Implemented Yet

- Registry fetch and artifact download.
- Integrity verification of downloaded content against lock checksums.
- Full transitive dependency graph solving.
- Hermetic compilation and artifact output reproducibility.

## Recommended Workflow

- Commit both `Dust.toml` and `dustpkg.lock`.
- Use `update` only when intentionally changing dependency resolution.
- Use explicit `--seed` in pipelines if lock ordering must be controlled.
