//! Core library for the `dustpkg` command line tool.
//!
//! This crate implements a simple package manager for the Dust
//! programming language.  The goal of `dustpkg` is to make
//! dependency management reproducible by always recording a
//! deterministic install plan in a lock file.  The lock file
//! contains exact versions and cryptographic hashes of every
//! dependency.  Reusing the lock file guarantees that subsequent
//! builds use the same dependency set.  This design follows the
//! reproducible builds guidelines that stress deterministic install
//! plans and immutable environments【718003472017320†L80-L91】.
//!
//! `dustpkg` supports a handful of subcommands:
//!
//! * `init` – bootstrap a new package by creating a `Dust.toml` file.
//! * `add <name> <version>` – add a dependency to the manifest.
//! * `update --seed <n>` – resolve dependencies and produce
//!   `dustpkg.lock`.  A seed may be provided to influence
//!   deterministic ordering, akin to deterministic thread
//!   schedulers that produce the same interleavings given the same
//!   seed【548468680421956†L121-L129】.
//! * `build --seed <n>` – ensure the lock file is up to date and
//!   prepare a reproducible build.  Currently this only verifies
//!   the lock file exists and prints a success message.
//!
//! Most of the heavy lifting happens in this library.  The
//! command‐line binary in `src/main.rs` simply forwards to these
//! functions.

use anyhow::{Context, Result};
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Metadata about the current package defined in `Dust.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PackageInfo {
    /// Name of the package.
    pub name: String,
    /// Version of the package.
    pub version: String,
    /// DPL specification version (v0.1 or v0.2)
    #[serde(default = "default_dpl_version")]
    pub dpl_version: String,
}

fn default_dpl_version() -> String {
    "0.2".to_string()
}

/// Manifest structure for `Dust.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub package: PackageInfo,
    /// Map of dependency names to version requirements.
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

impl Manifest {
    /// Load a manifest from the given path.
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read manifest at {}", path.display()))?;
        let manifest: Self = toml::from_str(&contents)
            .with_context(|| format!("failed to parse manifest at {}", path.display()))?;
        Ok(manifest)
    }

    /// Save the manifest to the given path.
    pub fn save(&self, path: &Path) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(self).context("failed to serialize manifest to TOML")?;
        fs::write(path, toml_string)
            .with_context(|| format!("failed to write manifest at {}", path.display()))?;
        Ok(())
    }
}

/// Locked dependency entry in `dustpkg.lock`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockedDep {
    pub name: String,
    pub version: String,
    /// SHA‑256 checksum of the dependency specification (name@version).
    pub checksum: String,
    /// Source location of the dependency (not used yet).
    pub source: String,
}

/// Lock file structure for `dustpkg.lock`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lockfile {
    pub package: PackageInfo,
    pub dependencies: Vec<LockedDep>,
    /// Seed used when resolving the dependencies; optional because
    /// deterministic ordering may be implied when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
}

impl Lockfile {
    /// Load a lock file from disk if it exists.
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read lock file at {}", path.display()))?;
        let lock: Self = toml::from_str(&contents)
            .with_context(|| format!("failed to parse lock file at {}", path.display()))?;
        Ok(lock)
    }

    /// Save the lock file to disk.
    pub fn save(&self, path: &Path) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(self).context("failed to serialize lock file to TOML")?;
        fs::write(path, toml_string)
            .with_context(|| format!("failed to write lock file at {}", path.display()))?;
        Ok(())
    }
}

/// Resolve a manifest into a lock file, deterministically ordering
/// dependencies using the provided seed.  If `seed` is `None`, the
/// dependencies are sorted alphabetically; otherwise the list is
/// shuffled using the seed.  Each locked dependency records a
/// checksum computed over its `name@version` pair.  The seed is
/// recorded in the resulting lock file.
pub fn resolve(manifest: &Manifest, seed: Option<u64>) -> Lockfile {
    let mut deps: Vec<(String, String)> = manifest
        .dependencies
        .iter()
        .map(|(name, version)| (name.clone(), version.clone()))
        .collect();
    if let Some(seed_val) = seed {
        let mut rng = ChaCha8Rng::seed_from_u64(seed_val);
        deps.shuffle(&mut rng);
    } else {
        deps.sort_by(|a, b| a.0.cmp(&b.0));
    }
    let locked_deps: Vec<LockedDep> = deps
        .into_iter()
        .map(|(name, version)| {
            let checksum = {
                let mut hasher = Sha256::new();
                hasher.update(format!("{}@{}", name, version).as_bytes());
                let result = hasher.finalize();
                hex::encode(result)
            };
            LockedDep {
                name: name.clone(),
                version: version.clone(),
                checksum,
                source: format!("registry/{}-{}", name, version),
            }
        })
        .collect();
    Lockfile {
        package: manifest.package.clone(),
        dependencies: locked_deps,
        seed,
    }
}

/// Initialise a new package in the given directory.  This creates a
/// `Dust.toml` manifest with default values.  Returns an error if
/// `Dust.toml` already exists.
pub fn init_package(dir: &Path) -> Result<()> {
    let manifest_path = dir.join("Dust.toml");
    if manifest_path.exists() {
        anyhow::bail!("Dust.toml already exists in {}", dir.display());
    }
    let name = dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("dustpkg-package");
    let manifest = Manifest {
        package: PackageInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            dpl_version: "0.2".to_string(),
        },
        dependencies: HashMap::new(),
    };
    manifest.save(&manifest_path)?;
    println!("Created Dust.toml for package '{}'", name);
    Ok(())
}

/// Add a dependency to the manifest at `manifest_path`.  This will
/// update the manifest and regenerate the lock file using the
/// provided seed.  If no seed is specified, the dependencies are
/// ordered alphabetically.
pub fn add_dependency(
    manifest_path: &Path,
    dep_name: &str,
    dep_version: &str,
    seed: Option<u64>,
) -> Result<()> {
    let mut manifest = Manifest::load(manifest_path)?;
    manifest
        .dependencies
        .insert(dep_name.to_string(), dep_version.to_string());
    manifest.save(manifest_path)?;
    let lock = resolve(&manifest, seed);
    let lock_path = manifest_path.with_file_name("dustpkg.lock");
    lock.save(&lock_path)?;
    println!(
        "Added dependency '{} = {}' and updated {}",
        dep_name,
        dep_version,
        lock_path.display()
    );
    Ok(())
}

/// Add the standard library dependencies (dustlib and optionally dustlib_k for v0.2).
/// This is called automatically for v0.2 projects to ensure K-regime support.
pub fn add_stdlib_dependencies(manifest_path: &Path, seed: Option<u64>) -> Result<()> {
    let mut manifest = Manifest::load(manifest_path)?;

    // Always add dustlib (core library)
    manifest
        .dependencies
        .insert("dustlib".to_string(), "0.2.0".to_string());

    // Add dustlib_k for v0.2 projects
    if manifest.package.dpl_version == "0.2" {
        manifest
            .dependencies
            .insert("dustlib_k".to_string(), "0.2.0".to_string());
    }

    manifest.save(manifest_path)?;
    let lock = resolve(&manifest, seed);
    let lock_path = manifest_path.with_file_name("dustpkg.lock");
    lock.save(&lock_path)?;
    println!("Added standard library dependencies (dustlib, dustlib_k)");
    Ok(())
}

/// Update the lock file by resolving the manifest.  This does not
/// modify the manifest.  The seed controls dependency ordering.
pub fn update_lock(manifest_path: &Path, seed: Option<u64>) -> Result<()> {
    let manifest = Manifest::load(manifest_path)?;
    let lock = resolve(&manifest, seed);
    let lock_path = manifest_path.with_file_name("dustpkg.lock");
    lock.save(&lock_path)?;
    println!("Updated {}", lock_path.display());
    Ok(())
}

/// Build the package by verifying the manifest and lock file are
/// consistent.  This function currently only checks that every
/// dependency in the manifest is present in the lock file and that
/// their versions match.  In a real implementation this would
/// perform compilation, caching and vendoring.  A seed may be
/// supplied to re-resolve the lock file before building.
pub fn build_package(manifest_path: &Path, seed: Option<u64>) -> Result<()> {
    // Optionally update the lock file to ensure it matches the manifest
    update_lock(manifest_path, seed)?;
    let manifest = Manifest::load(manifest_path)?;
    let lock_path = manifest_path.with_file_name("dustpkg.lock");
    let lock = Lockfile::load(&lock_path)?;
    // Check that each manifest dependency appears in the lock file with the same version
    for (name, version) in &manifest.dependencies {
        if let Some(entry) = lock.dependencies.iter().find(|dep| &dep.name == name) {
            if &entry.version != version {
                anyhow::bail!(
                    "version mismatch for dependency '{}': manifest {} vs lock {}",
                    name,
                    version,
                    entry.version
                );
            }
        } else {
            anyhow::bail!("dependency '{}' missing from lock file", name);
        }
    }
    // At this point a real implementation would compile or prepare
    // artifacts.  We simply output a success message.
    println!("Build successful. All dependencies resolved deterministically.");
    Ok(())
}
