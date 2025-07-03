#  Challenge

## 🚀 Challenge: Benchmarking Thread Pools

In this challenge, you'll implement **parameterized Criterion benchmarks** to evaluate the performance of your multithreaded key-value server using both your custom `SharedQueueThreadPool` and the `RayonThreadPool`, comparing the behavior of your `KvStore` and the `SledKvsEngine` under concurrent load.

This task involves setting up synthetic **write-heavy** and **read-heavy** workloads, varying the **number of threads** in your server's thread pool as a parameter, and measuring throughput across different configurations.

---

### 🚀 Goals

You must write **six benchmarks**:

#### ✅ 1. `write_queued_kvstore`
- Use `KvStore` with `SharedQueueThreadPool`.
- Benchmark how performance changes with different thread counts.
- Spawn 1000 client threads, each performing a `.set(...)` with a unique key.
- Verify correctness with assertions.
- Measure **only** the work of handling requests; setup must be outside `b.iter(...)`.

#### ✅ 2. `read_queued_kvstore`
- Use `KvStore` with `SharedQueueThreadPool`.
- Pre-load 1000 key-value pairs before measuring.
- Spawn 1000 client threads to perform `.get(...)` operations.
- Each should verify the value returned is correct.

#### ✅ 3. `write_queued_rayon`
- Same as benchmark #1, but use `RayonThreadPool`.

#### ✅ 4. `read_queued_rayon`
- Same as benchmark #2, but use `RayonThreadPool`.

#### ✅ 5. `write_queued_sled`
- Same as benchmark #3, but using `SledKvsEngine`.

#### ✅ 6. `read_queued_sled`
- Same as benchmark #4, but using `SledKvsEngine`.

---

### 🛠️ Technical Requirements

- Use `criterion::bench_function_over_inputs` to create parameterized benchmarks with varying thread counts.
- Vary the thread pool size: `1`, `2`, `4`, ..., up to **2× the number of logical CPU cores**.
- For each test, reuse the same keys and data across iterations for consistency.
- Benchmark throughput by measuring total request/response time across all threads.
- Avoid measuring setup/teardown inside the loop.
- Ensure thread synchronization is efficient — avoid bottlenecks that aren't part of the thread pool under test.

---

### 📊 Extra Credit

- Use the generated Criterion plots to:
  - Visualize the impact of thread count on throughput.
  - Compare scaling behavior between `SharedQueueThreadPool` and `RayonThreadPool`.
  - Discuss bottlenecks and scaling limits (e.g., thread contention, I/O limits, etc.).
- Provide a short writeup summarizing the observed performance trends.

---

### 📎 Hints

- Use `std::sync::mpsc` or `Arc<AtomicUsize>` to efficiently signal thread completion without affecting benchmark timing.
- Consider pre-spawning threads and reusing them across iterations to reduce overhead.
- If your `KvsClient` is blocking, you’ll need 1000 threads to send requests concurrently.


