# Clap 3 Challenge

Welcome to the **Clap 3** challenge, inspired by the [PingCap Talent Plan](https://github.com/pingcap/talent-plan). This is a bite-sized Rust project that introduces you to implementing a simple in-memory key-value store.

## 📚 Overview

This project currently uses [`structopt`](https://crates.io/crates/structopt) for parsing command-line arguments. However, `structopt` has been deprecated in favor of [`clap`](https://docs.rs/clap/latest/clap/) v3+, which includes a built-in derive macro system similar to `structopt`.

Your task is to **migrate the CLI argument parsing from `structopt` to `clap` v3+** using the modern `#[derive(clap::Parser)]` style.

---

### 🎯 Goals

- Replace all usage of `structopt` with `clap` v3+.
- Use `clap`'s derive macros to define the CLI structure.
- Preserve the same CLI interface:
  - `kvs set <key> <value>`
  - `kvs get <key>`
  - `kvs rm <key>`

---

### ✅ Acceptance Criteria

- `structopt` is removed from `Cargo.toml`.
- `clap` v3+ is added with the `derive` feature enabled.
- All CLI definitions use `#[derive(clap::Parser)]`, `#[command(...)]`, and `#[command(subcommand)]` as appropriate.
- The CLI behavior remains unchanged.
- The code builds and runs correctly with example commands.

---

### 🔎 Resources

- [Clap v4 Documentation](https://docs.rs/clap/latest/clap/)
- [Migrating from StructOpt](https://github.com/clap-rs/clap/blob/master/examples/derive_ref/structopt.rs)

---

Happy coding! 🚀
