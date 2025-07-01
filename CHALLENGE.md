# KVS-Server CLI Challenge

## 🧠 Challenge: Command-Line Parsing for `kvs-server`

Your task is to implement command-line argument parsing for a key-value store server application called `kvs-server`.

Use the [`clap`](https://docs.rs/clap/latest/clap/) crate to define a `Parser` that supports the following:

- A required `--addr` option (with default `127.0.0.1:4000`) to specify the IP and port the server should bind to.
- An optional `--engine` option that determines which storage engine to use (`kvs` or `sled`). If the option is not provided, no engine is selected by default.

Additionally:
- Define a `SocketAddr` for the `addr` field.
- Use a `ValueEnum` enum named `Engine` to represent the supported engine types.

**Note:** You’ll need to define `DEFAULT_LISTENING_ADDRESS` as a constant (`"127.0.0.1:4000"`) if not already present.

### 🔧 Hints

- You can add the `clap` crate to your dependencies in `Cargo.toml`:
  ```toml
  clap = { version = "4", features = ["derive"] }
  ```
- Make sure your enum derives `ValueEnum` and the appropriate traits for it to work with `clap`.

Good luck!

