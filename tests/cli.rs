use assert_cmd::prelude::*; // Provides extensions for running and asserting on binaries using Command
use predicates::ord::eq; // Used to compare exact string output (e.g., stdout equals "value1")
use predicates::str::{PredicateStrExt, contains, is_empty};
// `contains` matches substrings in output
// `is_empty` checks for completely empty stdout or stderr
// `PredicateStrExt` adds chaining and helper methods to string predicates
use std::process::Command; // Standard way to run and configure child processes
use tempfile::TempDir; // Creates automatically-cleaned temporary directories for test isolation

use kvs::{KvStore, Result};

// Test: Running `kvs` with no arguments should fail
#[test]
fn cli_no_args() {
    Command::cargo_bin("kvs") // Locates the compiled `kvs` binary using Cargo
        .unwrap() // Panics if the binary is not found (e.g., build failed)
        .assert() // Begin asserting on the command's output and status
        .failure(); // Asserts that the command exited with a non-zero status
}

// Test: `kvs -V` should print the version number to stdout
#[test]
fn cli_version() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["-V"]) // Pass `-V` as CLI argument to get version
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION"))); // Check that stdout includes the current version
}

// Test: `kvs get <KEY>` should print "Key not found" for a non-existent key and exit with zero.
// This checks that the program gracefully handles missing keys without erroring.
#[test]
fn cli_get_non_existent_key() {
    let temp_dir = TempDir::new().unwrap(); // Use a fresh temporary directory as isolated storage
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key1"]) // Try to get a non-existent key
        .current_dir(&temp_dir) // Ensure test does not affect or rely on user's actual files
        .assert()
        .success() // Getting a non-existent key is not an error; expect exit code 0
        .stdout(eq("Key not found").trim()); // Expected output
}

// Test: `kvs rm <KEY>` should print "Key not found" for an empty database and exit with non-zero code.
// This confirms that deleting a key that doesn’t exist is treated as an error.
#[test]
fn cli_rm_non_existent_key() {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["rm", "key1"]) // Try to remove a key that doesn't exist
        .current_dir(&temp_dir)
        .assert()
        .failure() // Removing a non-existent key should fail (exit code ≠ 0)
        .stdout(eq("Key not found").trim()); // Expected failure message
}

// Test: `kvs set <KEY> <VALUE>` should print nothing and exit with zero.
// This confirms that the `set` command works and is silent on success.
#[test]
fn cli_set() {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["set", "key1", "value1"]) // Set a key-value pair
        .current_dir(&temp_dir)
        .assert()
        .success() // Set should succeed
        .stdout(is_empty()); // Successful `set` produces no output
}

// Test: `kvs get <KEY>` should print value.
// This validates end-to-end integration between the library backend and the CLI.
#[test]
fn cli_get_stored() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    // Set up the store programmatically before invoking the CLI
    let mut store = KvStore::open(temp_dir.path())?;
    store.set("key1".to_owned(), "value1".to_owned())?;
    store.set("key2".to_owned(), "value2".to_owned())?;
    drop(store); // Explicitly close store to flush and unlock files

    // Now use the CLI to retrieve the values
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key1"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("value1").trim());

    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key2"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("value2").trim());

    Ok(())
}

// Test: `kvs rm <KEY>` should print nothing and exit with zero.
// This verifies that `rm` actually deletes the key and that `get` reflects this change.
#[test]
fn cli_rm_stored() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    // Insert a key so we can later remove it
    let mut store = KvStore::open(temp_dir.path())?;
    store.set("key1".to_owned(), "value1".to_owned())?;
    drop(store); // Close to ensure changes are saved to disk

    // Use CLI to remove the key
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["rm", "key1"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(is_empty()); // `rm` should produce no output on success

    // Use CLI to confirm the key is actually gone
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key1"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("Key not found").trim()); // Should now be missing
    Ok(())
}

// Test: Invalid usage of `get` (missing or too many args) should fail
#[test]
fn cli_invalid_get() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get"]) // Missing key
        .assert()
        .failure(); // Should return non-zero exit code

    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "extra", "field"]) // Too many arguments
        .assert()
        .failure();
}

// Test: Invalid usage of `set` (wrong number of args) should fail
#[test]
fn cli_invalid_set() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["set"]) // No key/value
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["set", "missing_field"]) // Missing value
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["set", "extra", "extra", "field"]) // Too many arguments
        .assert()
        .failure();
}

// Test: Invalid usage of `rm` (missing or too many args) should fail
#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["rm"]) // Missing key
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["rm", "extra", "field"]) // Too many arguments
        .assert()
        .failure();
}

// Test: Unknown subcommand should fail
#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["unknown", "subcommand"]) // Unknown CLI command
        .assert()
        .failure();
}
