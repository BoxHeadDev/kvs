#  Challenge

## 🚀 Challenge: Benchmarking

As the course progresses we will increasingly concern ourselves with the
performance of the database, exploring the impact of different architectures.
You are encouraged to go beyond the model described herein and experiment with
your own optimizations.

Performance work requires benchmarking, so now we're going to get started
on that. There are many ways to benchmark databases, with standard test
suites like [ycsb] and [sysbench]. In Rust benchmarking starts with
the builtin tooling, so we will start there.

[ycsb]: https://github.com/brianfrankcooper/YCSB
[sysbench]: https://github.com/akopytov/sysbench

Cargo supports benchmarking with `cargo bench`. The benchmarks may either be
written using Rust's built in benchmark harness, or an external one.

The built-in harness creates benchmarks from functions with the `#[bench]`
attribute. It cannot be used on the Rust stable channel though, and is only
documented briefly in [the unstable book][tb] and the [`test` crate docs][tc].
It is though widely used throughout the Rust ecosystem &mdash; crates that use
it, even if they compile with stable releases, do benchmarking with nightly
releases.

[tb]: https://doc.rust-lang.org/stable/unstable-book/library-features/test.html
[tc]: https://doc.rust-lang.org/stable/test/index.html

That system though is effectively deprecated &mdash; it is not being updated and
will seemingly never be promoted to the stable release channel.

There are better benchmark harnesses for Rust anyway. The one you will use is
[criterion]. And you will use it to satisfy your curiosity about the
performance of your `kvs` engine compared to the `sled` engine.

These benchmarking tools work by defining a benchmarking function, and within
that function iterating through a loop that performs the operation to be
benchmarking. The benchmarking tool will iterate as many times as it needs to in
order to know the duration of the operation with statistical significance.

See this basic example from the criterion guide:

```rust
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
	    b.iter(|| {
		    fibonacci(20)
		});
	});
}
```

The call to `bench_function` defines the benchmark, and the call to `iter`
defines the code that is run for the benchmark. Code before and after the call
to `iter` is not timed.

[criterion]: https://docs.rs/criterion

Prepare for writing benchmarks by creating a file called `benches/benches.rs`.
Like `tests/tests.rs`, cargo will automatically find this file and compile it as
a benchmark.

Start by writing the following benchmarks:

- `kvs_write` - With the kvs engine, write 100 values with random keys of length
  1-100000 bytes and random values of length 1-100000 bytes.

- `sled_write`- With the sled engine, write 100 values with random keys of
  length 1-100000 bytes and random values of length 1-100000 bytes.

- `kvs_read` - With the kvs engine, read 1000 values from previously written keys,
  with keys and values of random length.

- `sled_read` - With the sled engine, read 1000 values from previously written keys,
  with keys and values of random length.

(As an alternative to writing 4 benchmarks, you may also choose to write 2
benchmarks parameterized over the engine, as [described in the criterion
manual][pb]).

[pb]: https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html

These are underspecified, and there's a fair bit of nuance to implementing them
in a useful way. We need to consider at least three factors:

- What code should be timed (and be written inside the benchmark loop), and what
  code should not (and be written outside the benchmark loop)?

- How to make the loop run identically for each iteration, despite using
  "random" numbers.

- In the "read" benchmarks, how to read from the same set of "random" keys
  that were written previously.

These are all inter-related: some code needs to be carefully selected as
un-timed setup code, and the seed values for random number generators need
to be re-used appropriately.

In all cases, operations that may return errors should assert (with `assert!`)
that they did not return an error; and in the read case, "get" operations should
assert that the key was found.

Random numbers can be generated with the [`rand`] crate.

[`rand`]: https://docs.rs/crate/rand/

Once you have your benchmarks, run them with `cargo bench`.

_Write the above benchmarks, and compare the results between `kvs` and `sled`._

_Note: please run the benchmarks on an otherwise unloaded machine. Benchmark
results are very sensitive to the environment they are run in, and while the
criterion library does its best to compensate for "noise", benchmarks are best
done on a clean machine without other active processes. If you have a spare
machine just for development, use that. If not, an AWS or other cloud instance
may produce more consistent results than your local desktop.
