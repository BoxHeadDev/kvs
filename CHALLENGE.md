#  Challenge

## 🚀 Challenge: rayon threadpool

## 🧠 Challenge: Implement a Rayon-Based ThreadPool

Your task is to implement a `RayonThreadPool` that wraps the [`rayon::ThreadPool`](https://docs.rs/rayon/latest/rayon/struct.ThreadPool.html) and integrates it with the existing thread pool abstraction in your project.

### 🔧 Files to Create or Update

#### ✅ `rayon.rs`
Create a new file at `src/thread_pool/rayon.rs` (or wherever your `ThreadPool` trait is defined) and implement the `RayonThreadPool` struct so it adheres to the `ThreadPool` trait.

#### 🛠 `server.rs`
Update your server initialization code to optionally use RayonThreadPool in place of your default thread pool, based on configuration or a CLI flag (if applicable).

### 🧪 Goal: Pass This Test
Your implementation should pass the following test, typically found in `threadpool.rs`:

```rust
fn rayon_thread_pool_spawn_counter() -> Result<()> {
    let pool = RayonThreadPool::new(4)?;
    spawn_counter(pool)
}
```
The `spawn_counter` function checks that submitted jobs correctly run on the thread pool.

### 📝 Notes

- Add `rayon` to your `Cargo.toml`:
```rust
[dependencies]
rayon = "1.8"
```

- Make sure RayonThreadPool is included in your module tree, e.g., in mod.rs:
```rust
pub mod rayon;
```

- Ensure your ThreadPool trait is in scope and correctly implemented for RayonThreadPool.
