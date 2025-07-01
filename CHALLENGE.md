#  Challenge

## 🚀 Challenge: Build the Main Entry Point for `kvs-server`

You're now ready to put everything together and launch the key-value store server with a production-ready `kvs-server.rs`.

This binary will:
- Parse command-line arguments to configure the server (address and storage engine)
- Log startup configuration and runtime errors
- Open the key-value store engine
- Run the TCP server to handle incoming requests

## 🛠 Requirements

- Use `clap` to parse:
  - `--addr` to specify the listening socket address (default: `127.0.0.1:4000`)
  - `--engine` to optionally specify the storage engine (`kvs` or `sled`)
- Use the `log` crate to emit `info`, `warn`, and `error` logs.
- Initialize logging using `env_logger`.
- Gracefully handle errors and exit with code 1 if the server fails to launch.

## 🧩 Notes

- You’ll need the following dependencies in `Cargo.toml`:
  ```toml
  clap = { version = "4", features = ["derive"] }
  log = "0.4"
  env_logger = "0.10"
  ```
- The actual use of the `engine` argument is not implemented here — that would come later when supporting multiple engines like `sled`.
- This starter setup always uses `KvStore`, and logs key events for visibility.

With this complete, running `cargo run --bin kvs-server` will launch your storage server, ready to accept client requests!

🧱 Get ready to deploy your server in style!

