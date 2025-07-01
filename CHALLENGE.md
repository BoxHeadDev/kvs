# TCP Client Challenge

## 📡 Challenge: Implement a TCP Client for `KvsServer`

Your goal is to implement the networking layer for a key-value store client that communicates with a `KvsServer` over TCP. This client will be responsible for connecting to the server and setting up buffered input/output streams.

### 🧩 Task

Implement a `KvsClient` struct that:

- Maintains a buffered reader and writer for a TCP stream.
- Provides a `connect` method to establish a connection to a given socket address.

### 🛠 Requirements

- Use `BufReader` and `BufWriter` from `std::io` for efficient I/O.
- Use `TcpStream` from `std::net` to connect to the server.
- Ensure the `connect` method supports any type that implements `ToSocketAddrs`.

### 💡 Notes

- `try_clone()` is used to create a second handle to the TCP stream so that both reading and writing can occur independently.
- The `Result` type is assumed to be defined elsewhere in your project (typically a `Result<T, KvsError>` alias).
- The `common` module should contain your protocol request and response types like `Request`, `SetResponse`, etc.

Now go build your client-side logic!

