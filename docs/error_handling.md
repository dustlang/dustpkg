# Error Handling

## Error Model

`dustpkg` uses `anyhow::Result` and attaches path/context strings for I/O and parse operations.

Primary error sources:

- Missing files
- Invalid TOML format
- Serialization/write failures
- Manifest-lock validation failures

## Common User-Facing Errors

## Missing Manifest

For `add`, `update`, and `build`:

```text
Dust.toml not found in <cwd>
```

## Manifest Already Exists

For `init` in a non-empty project:

```text
Dust.toml already exists in <dir>
```

## Manifest/Lock Parse Failures

Typical context wrappers:

- `failed to read manifest at <path>`
- `failed to parse manifest at <path>`
- `failed to read lock file at <path>`
- `failed to parse lock file at <path>`

## Consistency Failures During Build

- `dependency '<name>' missing from lock file`
- `version mismatch for dependency '<name>': manifest <v1> vs lock <v2>`

## Operational Guidance

- Run `dustpkg update` after editing dependencies manually.
- Re-run `build` to validate lockfile alignment.
- Keep `Dust.toml` and `dustpkg.lock` committed together.
