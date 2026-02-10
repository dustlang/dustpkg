//! Executable entry point for the `dustpkg` command line tool.
//!
//! This binary provides a simple interface around the core
//! functionality exposed in `dustpkg::lib`.  It supports
//! subcommands for initialising packages, adding dependencies,
//! updating the lock file, and building packages.  Most work is
//! delegated to the library functions.

use clap::{Parser, Subcommand};
use dustpkg::{add_dependency, build_package, init_package, update_lock};
use std::path::PathBuf;

/// A deterministic package manager for the Dust programming language.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise a new Dust package in the current directory.
    Init,
    /// Add a dependency to the manifest.
    Add {
        /// Name of the dependency to add.
        name: String,
        /// Version requirement for the dependency.
        version: String,
        /// Optional seed to control dependency ordering in the lock file.
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Update the lock file to match the manifest.
    Update {
        /// Optional seed to control dependency ordering in the lock file.
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Build the current package.  This verifies the manifest and
    /// lock file are consistent, optionally using a seed to update
    /// the lock file first.
    Build {
        /// Optional seed to control dependency ordering in the lock file.
        #[arg(long)]
        seed: Option<u64>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("Dust.toml");
    match cli.command {
        Commands::Init => {
            init_package(&cwd)?;
        }
        Commands::Add { name, version, seed } => {
            if !manifest_path.exists() {
                anyhow::bail!("Dust.toml not found in {}", cwd.display());
            }
            add_dependency(&manifest_path, &name, &version, seed)?;
        }
        Commands::Update { seed } => {
            if !manifest_path.exists() {
                anyhow::bail!("Dust.toml not found in {}", cwd.display());
            }
            update_lock(&manifest_path, seed)?;
        }
        Commands::Build { seed } => {
            if !manifest_path.exists() {
                anyhow::bail!("Dust.toml not found in {}", cwd.display());
            }
            build_package(&manifest_path, seed)?;
        }
    }
    Ok(())
}