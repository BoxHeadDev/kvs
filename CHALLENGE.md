# Updated CLI Tests Challenge

Now that your `kvs` key-value store has a working CLI with real functionality, it's time to upgrade your test suite accordingly.

Your goal is to **replace stub-based tests** (which assume subcommands are "unimplemented") with **real integration tests** that validate the CLI's behavior end-to-end.

### ✅ Objectives

Update your existing CLI test suite to:

1. **Replace all tests expecting "unimplemented" messages** with tests that check:
   - `kvs set <KEY> <VALUE>` stores a value with no output and a zero exit code
   - `kvs get <KEY>` retrieves the correct value or prints `"Key not found"` if the key is missing
   - `kvs rm <KEY>` removes a key silently, or prints `"Key not found"` if it doesn't exist

2. **Use `tempfile::TempDir`** and `.current_dir()` to isolate each test’s database
   - This ensures no shared state across tests

3. **Add these new integration tests**:
   - `cli_get_stored`: Set multiple keys using `KvStore` API and verify retrieval via CLI
   - `cli_rm_stored`: Remove an existing key via CLI and verify it's gone

4. **Ensure all existing CLI argument validation tests still pass**

### 🧰 Tools You’ll Need

- `assert_cmd` — to run CLI commands and check exit codes and output
- `predicates` — to assert against `stdout` or `stderr`
- `tempfile` — to create isolated directories for testing
- `kvs::KvStore` — to pre-populate your database where needed

### 🧪 Example of the Updated Style

```rust
#[test]
fn cli_get_non_existent_key() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key1"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Key not found"));
}
```

### 🏁 Completion Criteria

- All your tests pass
- No test uses `.stderr(contains("unimplemented"))`
- New tests `cli_get_stored` and `cli_rm_stored` are present and correct
- Each test uses an isolated directory via `TempDir`

Good luck! Refactoring tests is just as important as writing them—and this will give you confidence in your CLI's real-world behavior.

