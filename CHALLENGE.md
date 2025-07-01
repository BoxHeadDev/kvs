# Request/Response Types Challenge

## 🔄 Challenge: Define Request and Response Types for KVS Protocol

In this challenge, you'll define the core request and response types used in communication between the key-value store client and server. These types will be serialized and deserialized using `serde` to support structured data transfer over the network.

## 🛠 Requirements

- Use `serde::{Serialize, Deserialize}` to enable serialization of all types.
- Define a `Request` enum that represents client operations (`Get`, `Set`, and `Remove`) with associated data.
- Define three response enums:
  - `GetResponse`: wraps either a value or an error message.
  - `SetResponse`: wraps a unit result or an error message.
  - `RemoveResponse`: wraps a unit result or an error message.

## 🔧 Notes

- You’ll need to include `serde` in your `Cargo.toml` with both `derive` and the appropriate `serde` features for your serialization backend (e.g. JSON):
  ```toml
  serde = { version = "1", features = ["derive"] }
  ```
- These enums will be used to serialize/deserialize messages between `KvsClient` and `KvsServer`.

Get ready to define your protocol for clean and robust communication!

