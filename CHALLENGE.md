#  Challenge

## 🧪 Challenge: Build a Full CLI Interface for `kvs-client`

In this challenge, you’ll design and implement a full-featured command-line interface for the `kvs-client` binary using the `clap` crate.

Your CLI should support three subcommands (`get`, `set`, and `rm`) to interact with the key-value store server. Each subcommand should send the appropriate request to the server over TCP and handle the response accordingly.

## 🛠 Requirements

- Use `clap::Parser` and `Subcommand` to define the structure of the CLI.
- Use `DEFAULT_LISTENING_ADDRESS` (`127.0.0.1:4000`) as the default server address.
- Handle all errors gracefully by printing to stderr and exiting with status code 1.
- Connect to the server using `KvsClient`, send the request, and print the result.

## 🧩 Tips

- Add `clap = { version = "4", features = ["derive"] }` to your `Cargo.toml`.
- Make sure to replace `Opt` with `Cli` or vice versa as needed for consistency.
- You can use `env!("CARGO_PKG_*")` variables to automatically fill in CLI metadata from `Cargo.toml`.

This will give you a fully functional client CLI that can read input, talk to the server, and display results. Ready to wire it up? 🔌

