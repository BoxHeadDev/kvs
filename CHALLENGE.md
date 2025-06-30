# Key-Value Get Challenge

## Challenge: Implement the `get` method for `KvStore`

The `KvStore` is a key-value store that persists commands in an append-only log.
This challenge is to implement the `get` method for `KvStore`.

### Background:
The store maintains a `BTreeMap<String, CommandPos>` index in memory, which
maps keys to their location in the log files. Each `CommandPos` contains:
- `file_id`: which generation file to look into,
- `pos`: byte offset to start reading from,
- `len`: length of the command in bytes.

Log files are accessed via the `readers` HashMap, which maps generation numbers to `BufReaderWithPos<File>`.

Each command in the log is a JSON-encoded enum `Command`, which can be either `Set` or `Remove`.

### Task:
Complete the `get` method to:
1. Check if the key exists in the index.
2. If it exists, use the corresponding reader to seek and read the `Set` command from disk.
3. Return the associated value as `Some(value)` if successful.
4. If the command found is not a `Set`, return `KvsError::UnexpectedCommandType`.
5. If the key does not exist, return `Ok(None)`.

### Signature:
```rust
pub fn get(&mut self, key: String) -> Result<Option<String>>;
```

### Hint:
Use `reader.seek(SeekFrom::Start(cmd_pos.pos))?` and `reader.take(cmd_pos.len)` to read the specific range from the log file.


