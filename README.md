# dustpkg – Dust Package Manager

`dustpkg` is an early prototype of a package manager for the
**Dust Programming Language (DPL)**.  Its primary goal is to
support **reproducible builds** by recording a deterministic
installation plan in a lock file.  When you run the same
commands on different machines or at different times, you
should end up with the exact same set of dependency versions.

## Motivation

Reproducible builds are vital for both security and developer
productivity.  A build is reproducible when it yields **exactly
the same output regardless of when or where it is run**【718003472017320†L80-L91】.  The
first step towards this is ensuring that dependency resolution is
deterministic.  Tools like Yarn, Cargo and Composer achieve this
by emitting a lock file that captures the complete dependency
graph【430168736340579†L79-L88】.  Without a lock file, installing
dependencies can pick up newer patch releases, leading to subtle
behaviour changes or even security compromises【430168736340579†L24-L76】.

`dustpkg` follows this philosophy.  It reads your `Dust.toml`
manifest and produces a `dustpkg.lock` file that pins every
dependency to an exact version along with a cryptographic
checksum.  Resolving dependencies with a lock file gives a
**deterministic install plan**, which is a key requirement for
repeatable builds【718003472017320†L80-L91】.

To further emphasise determinism, `dustpkg` optionally accepts a
**seed** when resolving dependencies.  The seed controls the
ordering of dependencies in the lock file.  This idea is
borrowed from deterministic schedulers used in property‑based
testing: given the same seed, the scheduler will produce the
same thread interleaving and therefore a reproducible result【548468680421956†L121-L129】.
While ordering doesn’t matter for functional correctness, it
makes the lock file fully deterministic and allows you to
generate different but reproducible builds for experimentation.

## Features

* `dustpkg init` – initialise a new package by creating a
  `Dust.toml` with the package’s name and default version.
* `dustpkg add <name> <version> [--seed <n>]` – add a
  dependency to the manifest and regenerate the lock file.  A
  seed controls deterministic ordering.
* `dustpkg update [--seed <n>]` – re‑resolve dependencies and
  update `dustpkg.lock` without modifying the manifest.
* `dustpkg build [--seed <n>]` – verify that the manifest and
  lock file are consistent.  In a future version this will
  compile your package and its dependencies in an isolated
  environment.  For now it simply checks that the versions
  recorded in the manifest and lock file match.

## Usage

1. Build the binary:

   ```bash
   cargo build --release
   ```

2. Initialise a new package in an empty directory:

   ```bash
   mkdir hello_dust
   cd hello_dust
   dustpkg init
   ```

   This creates a `Dust.toml` that looks like:

   ```toml
   [package]
   name = "hello_dust"
   version = "0.1.0"
   ```

3. Add dependencies:

   ```bash
   dustpkg add mathlib 1.2.3
   dustpkg add rand 0.4.1 --seed 42
   ```

   After each `add`, `dustpkg` writes a `dustpkg.lock` file
   recording the exact versions, a checksum for each dependency,
   and the seed used (if provided).  Committing both the
   manifest and lock file to version control allows team members
   to reproduce identical builds.

4. Update and build:

   ```bash
   dustpkg update
   dustpkg build
   ```

   `update` re‑resolves the manifest and writes a new lock file.
   `build` verifies that every dependency in the manifest matches
   the locked version and prints a success message.  A future
   version will also fetch and compile dependencies in a hermetic
   environment to ensure deterministic builds【718003472017320†L132-L170】.

## Reproducibility notes

`dustpkg` currently focuses on deterministic dependency
resolution via a lock file.  Full reproducibility also
requires an **immutable environment** and reliable access to
all build inputs【718003472017320†L132-L170】.  Future versions plan to
support:

* **Vendoring dependencies** – optionally fetching and
  storing dependency sources locally, so the build does not
  depend on external registries【718003472017320†L204-L214】.
* **Sandboxed builds** – running compilation in a container or
  virtual environment to remove variation in system libraries
  and environment variables【718003472017320†L146-L152】.
* **Hash‑based verification** – validating downloaded packages
  against the checksums recorded in the lock file.

For now, `dustpkg` demonstrates the core ideas and lays the
groundwork for a reproducible package ecosystem for Dust.

## License

`dustpkg` is licensed under the terms of the Dust Open Source
License.  See [`LICENSE`](LICENSE) for the full text.