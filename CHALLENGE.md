#  Challenge

## 🚀 Challenge: abstract threadpool

As in the previous project where you created a `KvsEngine` abstraction to compare different key-value engine implementations, now you are going to use the `ThreadPool` abstraction in a similar way—allowing interchangeable thread pool strategies (like `SharedQueueThreadPool`, `RayonThreadPool`, etc.).

### 🎯 Goal

Refactor your `KvServer` to support any implementation of the `ThreadPool` trait by making it generic over the thread pool type.

### 🛠️ Steps to Complete

#### 1. Add a Second Type Parameter for `ThreadPool`

Update the definition of `KvServer` to take a second generic type for the thread pool:

```rust
use crate::thread_pool::ThreadPool;
use crate::KvsEngine;

pub struct KvsServer<E: KvsEngine, P: ThreadPool> {
    engine: E,
    pool: P,
}
```

#### 2. Update the `KvServer` Constructor

Update the constructor method to accept the thread pool:

```rust
impl<E: KvsEngine, P: ThreadPool> KvsServer<E, P> {
    pub fn new(engine: E, pool: P) -> Self {
        KvsServer { engine, pool }
    }
}
```
#### 3. Use the Thread Pool to Distribute Work

Within your run method or wherever requests are handled, use self.pool.spawn(...) to delegate request handling tasks:
```rust
self.pool.spawn(move || {
    // Handle client connection, parse request, respond using `self.engine`
});
```
Note: You may need to clone or arc-wrap the engine if you're moving it into the thread closure.


