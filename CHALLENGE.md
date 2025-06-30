# CLI Test Challenge

Welcome to the **CLI Challenge**, a bite-sized task derived from the [PingCAP Talent Plan](https://github.com/pingcap/talent-plan). This challenge will help you practice building a simple command-line interface (CLI) tool using Rust and the [`clap`](https://docs.rs/clap/) crate.

## 🚀 Objective

Implement a basic key-value store CLI in Rust that supports three commands:

- `set <KEY> <VALUE>`: Set a string value for a string key.
- `get <KEY>`: Retrieve the value for a given key.
- `rm <KEY>`: Remove a key-value pair by key.

The starting code already defines the CLI interface using `clap`. Your task is to **write the internal logic** to make the commands work and pass the associated tests.

## 🧪 Behavior

Here is how the CLI is expected to behave:

### 1. Set a key

```sh
$ kvs set mykey myvalue
```

- Stores `"myvalue"` under the key `"mykey"`.

### 2. Get a key

```sh
$ kvs get mykey
myvalue
```

- Prints the value associated with `"mykey"`.
- If the key does not exist, print `"Key not found"` and exit with status code `1`.

### 3. Remove a key

```sh
$ kvs rm mykey
```

- Deletes the key-value pair.
- If the key does not exist, print `"Key not found"` and exit with status code `1`.

## 📦 Getting Started

### Requirements

- Rust (>= 1.70 recommended)

### Build the CLI

```sh
cargo build --release
```

### Run it

```sh
cargo run -- set foo bar
cargo run -- get foo
cargo run -- rm foo
```

## ✅ Tests

You can check your implementation with:

```sh
cargo test
```

