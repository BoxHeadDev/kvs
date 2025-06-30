use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

// `kvs` with no args should exit with a non-zero code.
// Expect the binary to exit with a failure status (non-zero exit code)
#[test]
fn cli_no_args() {}

// `kvs -V` should print the version
// Run `kvs -V` and check that the version output contains the package version
#[test]
fn cli_version() {}

// `kvs get <KEY>` should print "unimplemented" to stderr and exit with non-zero code
// Since `get` is not implemented yet, it should print an error and fail
#[test]
fn cli_get() {}

// `kvs set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_set() {}

// `kvs rm <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_rm() {}

// `kvs get` with missing or too many arguments should fail
#[test]
fn cli_invalid_get() {
    // Missing the key argument
    // Too many arguments
}

// `kvs set` with missing or too many arguments should fail
#[test]
fn cli_invalid_set() {
    // No arguments
    // Missing value
    // Too many arguments
}

// `kvs rm` with missing or too many arguments should fail
#[test]
fn cli_invalid_rm() {
    // No key provided
    // Too many arguments
}

// An unknown subcommand should result in failure
#[test]
fn cli_invalid_subcommand() {}
