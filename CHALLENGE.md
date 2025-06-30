# KvStore Tests Challenge

Welcome to the **KvStore Tests Challenge**, inspired by the [PingCAP Talent Plan](https://github.com/pingcap/talent-plan). This exercise is designed to build your skills in **test-driven development (TDD)** by writing and reasoning about tests **before** implementing functionality.

---

## 📋 Objective

Your goal is to write a series of unit tests for a simple key-value store, `KvStore`, *before* implementing the core logic. This exercise encourages thinking about API design, usage expectations, and correctness upfront.

---

## 📦 Project Structure

- All code and tests are written in **Rust**.
- You will define and test the behavior of the `KvStore` struct.
- No need to implement the internal logic yet — start by writing the tests!

---

## 🧪 What to Test

Below are the key behaviors you are expected to test. You will create **unit tests** to assert these behaviors.

### 1. Get Previously Stored Value

```rust
store.set("key1", "value1");
store.set("key2", "value2");
assert_eq!(store.get("key1"), Some("value1".to_owned()));
assert_eq!(store.get("key2"), Some("value2".to_owned()));
```

### 2. Overwrite Existing Value

```rust
store.set("key1", "value1");
store.set("key1", "value2");
assert_eq!(store.get("key1"), Some("value2".to_owned()));
```

### 3. Get Non-existent Key

```rust
store.set("key1", "value1");
assert_eq!(store.get("key2"), None);
```

### 4. Remove a Key

```rust
store.set("key1", "value1");
store.remove("key1");
assert_eq!(store.get("key1"), None);
```

---

## 🛠️ Getting Started

1. **Clone the repo** (or create your own minimal Rust project).
2. Create a test file: `tests/kv_store.rs`.
3. Write the tests listed above using `#[test]` and `assert_eq!`.
4. Use TDD: Do not implement `KvStore` until all tests are defined and written.
5. Implement `KvStore` to make all tests pass.

---

## 🚧 Constraints

- Do **not** use any external key-value libraries.
- You may use `std::collections::HashMap` internally.
- Focus on correctness and clarity.

---

## 🧠 Why This Matters

Writing tests before code:
- Encourages thoughtful API design.
- Helps catch logic errors early.
- Builds confidence in code changes.

This challenge simulates the real-world scenario of designing systems from the outside in, promoting a robust and testable architecture.

---

## ✅ Bonus

Once the in-memory store works, consider:
- Writing CLI tests using `assert_cmd`.
- Persisting data to disk.
- Handling errors gracefully.

---

Happy testing! 🧪🚀

