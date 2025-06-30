# Key-Value Set Challenge

🚀 Challenge: Persist Key-Value Operations to Disk

You've transitioned from an in-memory `KvStore` to a log-structured, file-backed key-value store. 
Your new implementation uses a write-ahead log for durability. 

🔧 Task:
Complete the transition by doing the following:

1. **Remove the use of the old in-memory `HashMap` (`map`)** from `KvStore::new` and `get`/`remove` methods.
    - Instead, use the `index` (`BTreeMap<String, CommandPos>`) for key lookups.
    - Implement `get(&self, key: String)` so it:
      - Looks up the key in the `index`.
      - Uses the `readers` map to access the correct log file.
      - Seeks and deserializes the `Command` to return the current value.
 
2. **Implement `remove(&mut self, key: String) -> Result<()>`**:
   - Append a `Remove` command to the log.
   - Remove the key from the `index`.

3. **Fix `KvStore::new()`**:
   - It currently uses an outdated constructor.
   - Update it to initialize readers, writer, current generation number, and index from the disk.

🧠 Concepts to Consider:
- File seeking and position tracking (`BufReaderWithPos`, `CommandPos`)
- Log compaction (optional stretch goal)
- Error handling during file I/O and deserialization

🛠️ Bonus:
- Write unit tests that simulate disk persistence by creating temporary directories.

Hint: The `Command` enum likely has variants like `Set` and `Remove`. Use them to deserialize log entries.

