# CLI Reference

## Binary

`dustpkg`

## Global Form

```text
dustpkg <COMMAND> [OPTIONS]
```

## Commands

## `init`

Initialize a new package in the current directory.

```text
dustpkg init
```

Behavior:

- Creates `Dust.toml` with default package metadata.
- Fails if `Dust.toml` already exists.

## `add`

Add one dependency to manifest and regenerate lockfile.

```text
dustpkg add <name> <version> [--seed <u64>]
```

Arguments:

- `name`: dependency key.
- `version`: dependency version string.

Options:

- `--seed <u64>`: deterministic seed for dependency ordering.

Behavior:

- Requires `Dust.toml` to already exist.
- Updates `[dependencies]` in manifest.
- Rewrites `dustpkg.lock` using `resolve`.

## `update`

Regenerate lockfile from current manifest.

```text
dustpkg update [--seed <u64>]
```

Options:

- `--seed <u64>`: deterministic seed for dependency ordering.

Behavior:

- Requires `Dust.toml`.
- Does not change manifest.
- Rewrites `dustpkg.lock`.

## `build`

Validate manifest and lockfile consistency.

```text
dustpkg build [--seed <u64>]
```

Options:

- `--seed <u64>`: if provided, lockfile is updated first with this seed.

Behavior:

- Requires `Dust.toml`.
- Calls `update_lock` before validation.
- Ensures each manifest dependency exists in lockfile with matching version.
- Prints success on pass.

## Current Exit/Failure Conditions

Representative failures:

- `Dust.toml not found in <cwd>`
- `Dust.toml already exists in <dir>`
- Parse/read/write failures with path context
- `dependency '<name>' missing from lock file`
- `version mismatch for dependency '<name>'`
