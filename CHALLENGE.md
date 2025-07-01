# Client RPC Challenge

## 🔌 Challenge: Implement Full Client-Side RPC for `KvsClient`

You’ve previously defined the `Request` and `Response` types for the key-value store protocol. Now it’s time to integrate them into the `KvsClient` to implement full client-side remote procedure calls using `serde_json`.

## 🎯 Your Task

Update the `KvsClient` to:

- Serialize `Request` enums and send them over the TCP stream.
- Deserialize the appropriate response (`GetResponse`, `SetResponse`, or `RemoveResponse`) from the server.
- Use a `serde_json::Deserializer` with a buffered reader to efficiently parse incoming JSON data.

## 🔧 Notes

- Make sure you’ve included `serde` and `serde_json` in your `Cargo.toml`:
  ```toml
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  ```
- You’ll need to handle flushing the writer after each request to ensure the data is sent immediately.
- The deserializer allows you to efficiently stream multiple JSON values from a single connection.

With this complete, your client can now send requests and handle responses in a structured, type-safe way. Power up your KVS!

