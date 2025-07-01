# TCP Server Challenge

### 🖥️ Challenge: Build a TCP Server for `KvsServer`

Your task is to implement a basic TCP server for the key-value store system. The server will listen for incoming client connections and log any connection errors.

### 🛠 Requirements

- Define a `KvsServer` struct with a method `run` that:
  - Binds to the given socket address.
  - Accepts incoming TCP connections using a loop.
  - Logs connection errors using the `log` crate.

- Use `TcpListener` from `std::net` to handle the server socket.
- Ensure `run` is generic over any type that implements `ToSocketAddrs`.

### 🔧 Notes

- The `Result` type is assumed to be defined elsewhere in your crate (commonly as a custom alias like `type Result<T> = std::result::Result<T, KvsError>`).
- Logging should be done using the `log` crate. You can initialize it in your `main.rs` using something like `env_logger::init();`.
- This challenge sets up the networking loop, but does not yet handle client requests — you'll expand on this in future steps.

Start building your TCP server!

