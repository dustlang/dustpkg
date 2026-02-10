use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper to run the `dustpkg` binary in a temporary directory.
fn run_dustpkg(args: &[&str], dir: &Path) -> assert_cmd::assert::Assert {
    let mut cmd = assert_cmd::Command::cargo_bin("dustpkg").unwrap();
    cmd.current_dir(dir).args(args);
    cmd.assert()
}

#[test]
fn init_creates_manifest() {
    let tmp = TempDir::new().unwrap();
    // run init
    run_dustpkg(&["init"], tmp.path()).success();
    // check Dust.toml exists
    let manifest_path = tmp.path().join("Dust.toml");
    assert!(manifest_path.exists(), "Dust.toml should be created");
    let contents = fs::read_to_string(&manifest_path).unwrap();
    assert!(contents.contains("[package]"), "manifest should contain [package] section");
}

#[test]
fn add_dependency_and_lock() {
    let tmp = TempDir::new().unwrap();
    run_dustpkg(&["init"], tmp.path()).success();
    run_dustpkg(&["add", "serde", "1.0.0"], tmp.path()).success();
    // After adding, we should have both manifest and lock file
    let manifest_path = tmp.path().join("Dust.toml");
    let lock_path = tmp.path().join("dustpkg.lock");
    assert!(manifest_path.exists());
    assert!(lock_path.exists());
    // Check manifest includes dependency
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    assert!(manifest.contains("serde"), "serde should be listed in dependencies");
    // Check lock file includes dependency and checksum
    let lock = fs::read_to_string(&lock_path).unwrap();
    assert!(lock.contains("name = \"serde\""), "lock should include serde");
    assert!(lock.contains("checksum"), "lock should include checksum");
}

#[test]
fn build_after_add() {
    let tmp = TempDir::new().unwrap();
    run_dustpkg(&["init"], tmp.path()).success();
    run_dustpkg(&["add", "serde", "1.0.0"], tmp.path()).success();
    // build should succeed
    run_dustpkg(&["build"], tmp.path()).success();
}

#[test]
fn update_with_seed_produces_different_order() {
    let tmp = TempDir::new().unwrap();
    run_dustpkg(&["init"], tmp.path()).success();
    // Add two dependencies
    run_dustpkg(&["add", "a", "0.1.0"], tmp.path()).success();
    run_dustpkg(&["add", "b", "0.2.0"], tmp.path()).success();
    // update with seed 0
    run_dustpkg(&["update", "--seed", "0"], tmp.path()).success();
    let lock_seed0 = fs::read_to_string(tmp.path().join("dustpkg.lock")).unwrap();
    // update with seed 42
    run_dustpkg(&["update", "--seed", "42"], tmp.path()).success();
    let lock_seed42 = fs::read_to_string(tmp.path().join("dustpkg.lock")).unwrap();
    // If seed influences ordering, the two lock contents should differ
    assert_ne!(lock_seed0, lock_seed42, "different seeds should produce different lock ordering");
}