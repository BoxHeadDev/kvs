#  Challenge

## 🚀 Challenge: Shared KvsEngine

Before we can integrate the `NaiveThreadPool` into `KvServer` we have to make
the `KvsEngine` trait and the `KvStore` implementation (for now you can ignore
the `SledKvsEngine` from the previous project, but you can optionally
re-implement it as an extension to this project).

Recall from the project spec that, this time, our `KvsEngine` takes `self` as
`&self` instead of `&mut self` as previously, It also implements `Clone`, which
must be done explicitly for each implementation, as well as `Send + 'static`,
implicit properties of the definition of each implementation. More concretely,
it looks like

```rust
pub trait KvsEngine: Clone + Send + 'static {
    fn set(&self, key: String, value: String) -> Result<()>;

    fn get(&self, key: String) -> Result<Option<String>>;

    fn remove(&self, key: String) -> Result<()>;
}
```

This gives us a lot of clues about the implementation strategy we're pursuing.
First, think about why the engine needs to implement `Clone` when we have a
multithreaded implementation. Consider the design of other concurrent data
types in Rust, like [`Arc`]. Now think about why that makes us use `&self`
instead of `&mut self`. What do you know about shared mutable state? By the end
of this project be sure you understand the implications here &mdash; _this is
what Rust is all about_.

[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html

In this model, `KvsEngine` behaves like a _handle_ to another object, and
because that object is shared between threads, it probably needs to live on the
[heap], and because that shared state can't be mutable it needs to be protected by
some synchronization primitive.

[heap]: https://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap

So, _move the data inside your implementation of `KvsEngine`, `KvStore` onto
the heap using a thread-safe shared pointer type and protect it behind a lock of
your choosing_.

Since `SledKvsEngine` implements `KvsEngine` it may also need to change.

At this point your single-threaded `kvs-server` should work once again, but now
with a `KvsEngine` that can later be shared across threads.

**Test cases to complete**:

  - `kv_store::concurrent_*`
