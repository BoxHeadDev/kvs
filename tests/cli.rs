use assert_cmd::prelude::*; // Provides extensions for running and asserting on binaries
use predicates::str::contains; // Provides predicates for matching strings in output
use std::process::Command;

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

// Test: `kvs get <KEY>` should fail and print "unimplemented" to stderr
#[test]
fn cli_get() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key1"]) // Provide `get` subcommand and a key
        .assert()
        .failure() // Expect a non-zero exit code
        .stderr(contains("unimplemented")); // Expect "unimplemented" in stderr
}

// Test: `kvs set <KEY> <VALUE>` should fail and print "unimplemented" to stderr
#[test]
fn cli_set() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["set", "key1", "value1"]) // Provide `set` subcommand with key-value
        .assert()
        .failure()
        .stderr(contains("unimplemented")); // Should indicate unimplemented
}

// Test: `kvs rm <KEY>` should fail and print "unimplemented" to stderr
#[test]
fn cli_rm() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["rm", "key1"]) // Provide `rm` subcommand with key
        .assert()
        .failure()
        .stderr(contains("unimplemented")); // Should indicate unimplemented
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
