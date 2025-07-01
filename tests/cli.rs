use assert_cmd::prelude::*;
use predicates::str::{contains, is_empty};
use std::fs::{self, File};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// `kvs-client` with no args should exit with a non-zero code.
#[test]
fn client_cli_no_args() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("kvs-client").unwrap();
    cmd.current_dir(&temp_dir).assert().failure();
}

#[test]
fn client_cli_invalid_get() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "extra", "field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "key", "--addr", "invalid-addr"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "key", "--unknown-flag"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

#[test]
fn client_cli_invalid_set() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "missing_field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "key", "value", "extra_field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "key", "value", "--addr", "invalid-addr"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "key", "--unknown-flag"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

#[test]
fn client_cli_invalid_rm() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm", "key", "--addr", "invalid-addr"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm", "key", "--unknown-flag"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

#[test]
fn client_cli_invalid_subcommand() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["unknown"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

// `kvs-client -V` should print the version
#[test]
fn client_cli_version() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("kvs-client").unwrap();
    cmd.args(&["-V"])
        .current_dir(&temp_dir)
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

// `kvs-server -V` should print the version
#[test]
fn server_cli_version() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("kvs-server").unwrap();
    cmd.args(&["-V"])
        .current_dir(&temp_dir)
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}
