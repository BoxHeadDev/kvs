# Key-Value Remove Challenge

## Challenge: Implement the `remove` method for `KvStore`

The `KvStore` is a persistent key-value store that logs all operations to disk
using an append-only log format. Keys and their corresponding positions in
the log are stored in the in-memory `index`.

### Background:
The store supports removing keys by appending a `Remove` command to the log.
To do this, we must:
- Ensure the key exists in the in-memory index.
- Log a `Command::Remove` for that key.
- Remove the key from the in-memory index.

### Task:
Implement the `remove` method that:
1. Checks if the key exists in `self.index`.
2. If it doesn't exist, return `Err(KvsError::KeyNotFound)`.
3. If it does exist, serialize and write a `Remove` command to the writer.
4. Flush the writer to ensure it's written to disk.
5. Remove the key from the `index`.

### Signature:
```rust
pub fn remove(&mut self, key: String) -> Result<()>
```

### Notes:
- The `index` contains the latest known position for each key on disk.
- You **must** remove the key from the index after appending the `Remove` command.
- If the key is missing, return `KvsError::KeyNotFound`.

