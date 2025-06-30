# Compaction Test Challenge

Implement and verify a compaction mechanism in `KvStore` such that:
- Repeatedly setting the same keys with new values eventually triggers compaction.
- Compaction is defined as a measurable **decrease** in the total size of the data directory.
- After compaction, reopening the store should yield correct values for all keys (latest written).

## Requirements:
- Insert key-value pairs repeatedly with changing values.
- Monitor the directory size after each iteration.
- Detect when the directory size decreases, indicating compaction occurred.
- After compaction is detected:
    - Drop and reopen the store.
    - Assert that all keys return the latest values.
- If no compaction occurs after sufficient data insertion, the test must fail.

