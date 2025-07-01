#  Challenge

## 🚀 Challenge: Pluggable Storage Engines

Your database has a storage engine, `KvStore`, implemented by you.
Now you are going to add a second storage engine.

There are multiple reasons to do so:

- Different workloads require different performance characteristics. Some
  storage engines may work better than other based on the workload.

- It creates a familiar framework for comparing different backends.

- It gives us an excuse to create and work with traits.

- It gives us an excuse to write some comparative benchmarks!

So you are going to _extract_ a new trait, `KvsEngine`, from the `KvStore`
interface. This is a classic _refactoring_, where existing code is transformed
into a new form incrementally. When refactoring you will generally want to break
the work up into the smallest changes that will continue to build and work.

Here is the API you need to end up with:

- `trait KvsEngine` has `get`, `set` and `remove` methods with the same signatures
  as `KvStore`.

- `KvStore` implements `KvsEngine`, and no longer has `get`, `set` and `remove`
  methods of its own.

- There is a new implementation of `KvsEngine`, `SledKvsEngine`. You need to fill
  its `get` and `set` methods using the `sled` library later.

It's likely that you have already stubbed out the definitions for these if your
tests are building. _Now is the time to fill them in._ Break down your
refactoring into an intentional sequence of changes, and make sure the project
continues to build and pass previously-passing tests before continuing.

As one final step, you need to consider what happens when `kvs-server` is
started with one engine, is killed, then restarted with a different engine. This
case can only result in an error, and you need to figure out how to detect the
case to report the error. The test `cli_wrong_engine` reflects this scenario.

