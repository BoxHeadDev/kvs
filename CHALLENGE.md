# CLI Test Challenge

Welcome to the first step in building your own key-value store as part of a PingCap Talent Plan-inspired project! 🎯

This challenge focuses on writing **CLI tests** for a `kvs` binary. The goal is to ensure the program handles various CLI invocations correctly — including input validation, basic structure, and placeholder responses for unimplemented functionality.

---

## 🔍 What You’ll Be Testing

You will write unit tests using the [`assert_cmd`](https://docs.rs/assert_cmd) and [`predicates`](https://docs.rs/predicates) crates to verify that the `kvs` CLI behaves as expected. These tests are meant to run against a **starter binary** that has not yet implemented the actual key-value logic.

### CLI Behavior to Validate

| Command | Expected Behavior |
|--------|--------------------|
| `kvs` (no args) | Fails with non-zero exit code |
| `kvs -V` | Prints version (from `CARGO_PKG_VERSION`) |
| `kvs get <KEY>` | Fails, prints `"unimplemented"` to stderr |
| `kvs set <KEY> <VALUE>` | Fails, prints `"unimplemented"` to stderr |
| `kvs rm <KEY>` | Fails, prints `"unimplemented"` to stderr |
| Invalid argument counts (too few or too many) | Fails |
| Unknown subcommands | Fails |

---

## ✅ How to Run the Tests

You can run the CLI tests using:

```bash
cargo test
```
---

Ensure you have the following dependencies added in your Cargo.toml under [dev-dependencies]:

```
assert_cmd = "2.0"
predicates = "3.0"
```

Your project structure should contain a test file like tests/cli.rs with all the test cases.

---

## 🚀 Goal of This Challenge

- Practice writing robust CLI tests
- Learn to use assert_cmd and predicates effectively
- Lay the foundation for future CLI and backend logic
