# Updated CLI Tests Challenge

**Challenge:** Upgrade the KvStore Test Suite for Persistence and Error Handling

The current tests for the `KvStore` are written to check basic in-memory behavior.
Your goal is to **refactor** these tests to ensure the key-value store is also
persistent across restarts and returns appropriate `Result` types.

### Tasks:

1. **Use Temporary Storage**:
   - Replace in-memory store creation (`KvStore::new()`) with persistent store creation using a temporary directory (`KvStore::open(temp_dir.path())?`).
   - Use `TempDir` from the `tempfile` crate to create a temporary directory for each test.

2. **Handle Results Properly**:
   - Update each test function signature to return `Result<()>`.
   - Properly propagate errors using the `?` operator after each fallible method call (`set`, `get`, `remove`, `open`).

3. **Add Recovery Verification**:
   - After writing data to the store, `drop` the store and reopen it to ensure data is actually written to disk and can be recovered.

4. **Add Test for Non-existent Key Removal**:
   - Add a test that tries to remove a non-existent key and confirms it returns an error.

### Original Tests to Refactor:

- `get_stored_value`
- `overwrite_value`
- `get_non_existent_value`
- `remove_key`

### New Test to Add:

- `remove_non_existent_key`

### Example Hint:

Here's how the beginning of your updated `get_stored_value` test might look:

```rust
#[test]
fn get_stored_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    store.set("key1".to_owned(), "value1".to_owned())?;
    ...
```

Use this pattern across all your tests.

**Goal**: Ensure your `KvStore` implementation passes all updated tests, demonstrating correctness and durability.


