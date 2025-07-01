# Server RPC Challenge

## 🧠 Challenge: Handle Requests and Send Responses in `KvsServer`

You’ve already set up your request/response types and built the networking skeleton for your key-value store server. Now it’s time to complete the server logic to:

- Accept and deserialize incoming client requests (`Get`, `Set`, `Remove`)
- Call the appropriate methods on the `KvStore` engine
- Serialize and return the appropriate response types

## 🎯 Your Task

Update the `KvsServer` implementation to:

- Deserialize `Request` values from the TCP stream using `serde_json::Deserializer`
- Match each `Request` variant and invoke the corresponding method on the `KvStore`
- Serialize and send the correct response type (`GetResponse`, `SetResponse`, `RemoveResponse`) back to the client
- Log each request and response using the `log` crate

## 🧩 Notes

- `KvStore` is assumed to implement `get`, `set`, and `remove` methods returning your project’s `Result` type.
- Use `serde_json::Deserializer` with `into_iter::<Request>()` to stream multiple requests over a single connection.
- Don’t forget to initialize the logger in your `main.rs`:
  ```rust
  env_logger::init();
  ```
- You’ll need to add dependencies in your `Cargo.toml`:
  ```toml
  serde_json = "1"
  log = "0.4"
  env_logger = "0.10"
  ```

Build your server to process real client requests — one line of JSON at a time!

