# Key-Value Cli Challenge

## Challenge: Connect `KvStore` to an Existing CLI Interface

This challenge builds upon an existing CLI definition using `clap` v2.
Your task is to implement the logic that connects each CLI subcommand
to the corresponding method on the `KvStore` type.

### Given:
- A CLI is already defined with subcommands: `set`, `get`, `rm`.
- The `KvStore` API is available with:
  - `KvStore::open(path: impl Into<PathBuf>) -> Result<KvStore>`
  - `set(key: String, value: String) -> Result<()>`
  - `get(key: String) -> Result<Option<String>>`
  - `remove(key: String) -> Result<()>`

### Goals:
1. Open the store in the current directory using `KvStore::open(current_dir()?)`.
2. For `set`, retrieve `KEY` and `VALUE` arguments and call `set()`.
3. For `get`, retrieve `KEY`, call `get()`, and print the value or `Key not found`.
4. For `rm`, retrieve `KEY`, call `remove()`, and if the key is missing, print `Key not found` and exit with code `1`.

