use assert_cmd::prelude::*; // Provides extensions for running and asserting on binaries using Command
use predicates::ord::eq; // Used to compare exact string output (e.g., stdout equals "value1")
use predicates::str::{PredicateStrExt, contains, is_empty};
// `contains` matches substrings in output
// `is_empty` checks for completely empty stdout or stderr
// `PredicateStrExt` adds chaining and helper methods to string predicates
use std::process::Command; // Standard way to run and configure child processes
use tempfile::TempDir; // Creates automatically-cleaned temporary directories for test isolation
use walkdir::WalkDir; // Recursively walks directories — useful for inspecting files, though not used in the current code

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
    // - Create a new temporary directory for isolated storage
    // - Run the CLI command `kvs get key1` within that directory
    // - Assert that the command exits successfully (exit code 0)
    // - Assert that the output to stdout is exactly "Key not found" (no extra whitespace)
}

// Test: `kvs rm <KEY>` should print "Key not found" for an empty database and exit with non-zero code.
// This confirms that deleting a key that doesn’t exist is treated as an error.
#[test]
fn cli_rm_non_existent_key() {
    // - Use a temporary directory to ensure an empty key-value store
    // - Run the CLI command `kvs rm key1` in that directory
    // - Assert that the command exits with a failure (non-zero exit code)
    // - Assert that stdout contains "Key not found"
}

// Test: `kvs set <KEY> <VALUE>` should print nothing and exit with zero.
// This confirms that the `set` command works and is silent on success.
#[test]
fn cli_set() {
    // - Create a temporary directory to act as the storage location
    // - Run the CLI command `kvs set key1 value1` inside that directory
    // - Assert that the command succeeds (exit code 0)
    // - Assert that stdout is completely empty
}

// Test: `kvs get <KEY>` should print value.
// This validates end-to-end integration between the library backend and the CLI.
#[test]
fn cli_get_stored() -> Result<()> {
    // - Create a temporary directory
    // - Open a `KvStore` in that directory and store multiple key-value pairs (e.g., key1 -> value1, key2 -> value2)
    // - Drop the store to flush changes
    // - Run `kvs get key1` and `kvs get key2` via CLI in that directory
    // - Assert that both commands succeed and return the correct values in stdout ("value1", "value2")
}

// Test: `kvs rm <KEY>` should print nothing and exit with zero.
// This verifies that `rm` actually deletes the key and that `get` reflects this change.
#[test]
fn cli_rm_stored() -> Result<()> {
    // - Create a temporary directory
    // - Open a `KvStore` and insert a key-value pair (e.g., key1 -> value1), then drop the store to save changes
    // - Use the CLI to run `kvs rm key1` in that directory and assert it succeeds with no output
    // - Then run `kvs get key1` via CLI and assert it succeeds with output "Key not found"
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
