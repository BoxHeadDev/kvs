# Compaction Challenge

## 🧠 CHALLENGE: Implement Log Compaction in `KvStore`

## GOAL:
Add a log compaction mechanism to the KvStore to prevent the log files from growing indefinitely.
Compaction should remove obsolete commands and consolidate the latest key-value pairs into a new log file.

## STEP-BY-STEP GUIDE:

1. Add a new field to `KvStore`:
   - `uncompacted: u64` to track how many bytes can be reclaimed via compaction.

2. Update the `load` function to return the number of bytes considered "uncompacted".
   - This includes any overwritten or deleted commands while reconstructing the index.

3. Modify `KvStore::open`:
   - Accumulate the returned uncompacted bytes during log replay.
   - Store the total in the `uncompacted` field.

4. Modify `set` and `remove` methods:
   - Whenever an old command is overwritten or a key is deleted, increment `uncompacted`.
   - If `uncompacted > COMPACTION_THRESHOLD` (e.g., 1MB), trigger `self.compact()?`.

5. Implement `KvStore::compact`:
   - Write all live entries (from `self.index`) to a new compaction log file.
   - Update all `CommandPos` entries to point to the new file/positions.
   - Delete older log files (`file_id < compaction_file_id`).
   - Reset `uncompacted = 0`.

6. Add a helper method `new_log_file` on `KvStore`:
   - It wraps the global `new_log_file(...)` and updates `self.readers`.

7. Add `COMPACTION_THRESHOLD` constant:
   ```rust
   const COMPACTION_THRESHOLD: u64 = 1024 * 1024; // 1MB
   ```

## BONUS:
- Write a test to confirm that old log files are deleted after compaction.
- Ensure `set`/`get`/`remove` still work correctly post-compaction.

🏁 Once complete, your `KvStore` will support efficient, space-reclaiming log compaction!

