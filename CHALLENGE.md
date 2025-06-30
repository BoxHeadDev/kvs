# Key-Value Open Challenge

## Challenge: Implement the `open` method for `KvStore`

The `KvStore` persists key-value pairs in log files. When opening a store from disk,
we must reconstruct the store's state by inspecting all past logs and preparing
for future writes.

### Background:
- Each log file is named with a generation number (e.g., `1.log`, `2.log`).
- The `KvStore` uses an in-memory index to map keys to their latest command position.
- When opening the store, all `.log` files must be scanned in order to reconstruct
  this index and prepare a writer for the next generation log file.

### Goals:
Implement the `KvStore::open` function along with these helper functions:

#### `log_path(dir: &Path, file_id: u64) -> PathBuf`
- Returns the full path to the log file for a given generation number.

#### `sorted_file_list(path: &Path) -> Result<Vec<u64>>`
- Scans the directory for `.log` files,
- Extracts and sorts their generation numbers,
- Returns the sorted list.

#### `new_log_file(...) -> Result<BufWriterWithPos<File>>`
- Creates a new `.log` file for writing,
- Adds its corresponding `BufReaderWithPos` to the `readers` map,
- Returns a new `BufWriterWithPos` for writing new commands.

#### `KvStore::open(path: impl Into<PathBuf>) -> Result<KvStore>`
- Creates the directory if it doesn't exist,
- Initializes the log readers and the writer,
- Sets `current_file` to the next available generation number,
- Returns a fully initialized `KvStore`.

