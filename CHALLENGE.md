#  Challenge

## 🚀 Challenge: shared queue threadpool

So now that you've got your multithreaded architecture in place, it's time to
write a real thread pool. You probably wouldn't write your own thread pool in
practice as there exist thread pool crates that are well-tested, but it
is a useful exercise to gain experience with concurrency in general. Later in
this project you will, as we did with the engine in the previous project,
abstract the thread pool and compare the performance of yours with an existing.

So, what is a thread pool?

It's nothing complicated. Instead of creating a new thread for every
multithreaded job to be performed, a thread pool maintains a "pool" of threads,
and reuses those threads instead of creating a new one.

But why?

It's entirely about performance. Reusing threads saves a small amount of
performance, but when writing high-performance applications, every bit counts.
Imagine what it takes to make a new thread:

You've got to have a call stack for that thread to run on. That call stack must
be allocated. Allocations are pretty cheap, but not as cheap as no allocation.
How that call stack is allocated depends on details of the operating system and
runtime, but can involve locks and syscalls. Syscalls again are not _that_
expensive, but they are expensive when we're dealing with Rust levels of
performance &mdash; reducing syscalls is a common source of easy optimizations.
That stack then has to be carefully initialized so that first [stack frame]
contains the appropriate values for the base pointer and whatever else is needed
in the stack's initial [function prologue][fp]. In Rust the stack needs to be
configured with a [guard page] to prevent stack overflows, preserving memory
safety. That takes two more syscalls, [to `mmap` and to `mprotect`][mp] (though
on Linux in particular, those two syscalls are avoided).

[guard page]: https://docs.microsoft.com/en-us/windows/desktop/Memory/creating-guard-pages
[fp]: https://en.wikipedia.org/wiki/Function_prologue
[stack frame]: https://en.wikipedia.org/wiki/Stack_frame
[2mb]: https://github.com/rust-lang/rust/blob/6635fbed4ca8c65822f99e994735bd1877fb063e/src/libstd/sys/unix/thread.rs#L12
[mp]: https://github.com/rust-lang/rust/blob/6635fbed4ca8c65822f99e994735bd1877fb063e/src/libstd/sys/unix/thread.rs#L315

<!-- TODO: illustration? -->

That's just setting up the callstack. It's at least another syscall to create
the new thread, at which point the kernel must do its own internal accounting
for the new thread.

In Rust, the C [libpthread] library handles most of this complexity.

Then at some point the OS performs a [context switch] onto the new stack, and
the thread runs. When the thread terminates all that work needs to be undone
again.

With a thread pool, all that setup overhead is only done for a few threads, and
subsequent jobs are simply context switches into existing threads in the pool.

[libpthread]: https://www.gnu.org/software/hurd/libpthread.html
[context switch]: https://en.wikipedia.org/wiki/Context_switch


### So how do you build a thread pool?

There are many strategies and tradeoffs, but for this exercise you are going to
use a single shared queue to distribute work to idle threads. That means that
your "producer", the thread that accepts network connections, sends jobs to a
single queue (or channel), and the "consumers", every idle thread in the pool,
read from that channel waiting for a job to execute. This is the very simplest
work scheduling strategy, but it can be effective. What are the downsides?

You have three important considerations here:

1) _which data structure to use to distribute the work_ &mdash; it's going to be a
  queue, and there is going to be one sender ("producer"), the thread listening
  for TCP connections, and many recievers ("consumers"), the threads in the pool.

2) _how to deal with panicking jobs_ &mdash; your pool runs arbitrary work items.
  If a thread panics, the thread pool needs to recover in some way.

3) _how to deal with shutdown_ &mdash; when the `ThreadPool` object goes out of
  scope it needs to shut down every thread. It must not leave them idle.

These concerns are all intertwined since dealing with each of them may involve
communication and synchronization between threads. Some solutions will be
simple, the solutions to each of these working together gracefully; some
solutions will be complex, the solutions being independent and convoluted.
Choose your data structures carefully and use their capabilities wisely.

You will distribute work by sending messages over some concurrent queue type (a
concurrent queue in Rust typically being a data structure with two connected
types: sender types, and reciever types; and that can send between the two types
any type that implements `Send` + `'static`).

Messages in Rust are typically represented as enums, with variants for each
possible message that can be sent, like:

```rust
enum ThreadPoolMessage {
    RunJob(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}
```

This tends to be a simpler and more efficient solution than trying to "juggle"
multiple channels for different purposes. Of course, if there is only one type
of message, an enum is not necessary. Now, the above example may or may not be
the full set of messages you need to manage your thread pool, depending on the
design. In particular, shutting down can often be done implicitly if your queue
returns a result indicating that the sender has been destroyed.

There are many types of multithreaded queues. In Rust the most common is the
[`mpsc`] channel, because it lives in Rust's standard library. This is a
multi-producer, single consumer queue, so using it for your single-queue thread
pool will require a lock of some kind. What's the downside of using a lock here?
There are many other concurrent queue types in Rust, and each has pros and cons.
If you are willing to take a lock on both producer and consumer sides, then you
could even use a `Mutex<VecDeque>`, but there's probably no reason to do that in
production when better solutions exist.

[`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html

_Historical note: the existence of channels in Rust's standard library is a bit
of a curiosity, and is considered a mistake by some, as it betrays Rust's
general philosophy of keeping the standard library minimal, focused on
abstracting the operating system, and letting the crate ecosystem experiment
with advanced data structures. Their presence is an artifact of Rust's
development history and origins as a message-passing language like Go. Other
libraries like [`crossbeam`] provide more sophisticated alternatives, and
sometimes more suitable options_ 😉.

[`crossbeam`]: https://github.com/crossbeam-rs/crossbeam

Your thread pool will need to deal with the case where the spawned function
panics &mdash; simply letting panics destroy the threads in your pool would
quickly deplete its available threads. So if a thread in your pool panics you
need to make sure that the total number of threads doesn't decrease. So what
should you do? You have at least two options: let the thread die and spawn
another, or catch the panic and keep the existing thread running. What are the
tradeoffs? You've got to pick one, but leave a comment in your code explaining
your choice.

Some of the tools at your disposal are [`thread::spawn`], [`thread::panicking`],
[`catch_unwind`], [`mpsc`] channels, [`Mutex`], [crossbeam's MPMC
channels][mpmc], and `thread`s [`JoinHandle`]. You may use any of these, but
probably not all.

[`thread::spawn`]: https://doc.rust-lang.org/std/thread/fn.spawn.html
[`thread::panicking`]: https://doc.rust-lang.org/std/thread/fn.panicking.html
[`catch_unwind`]: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html
[`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[mpmc]: https://docs.rs/crossbeam/0.7.1/crossbeam/channel/index.html
[`JoinHandle`]: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html

_Create the `SharedQueueThreadPool` type, implementing `ThreadPool`_.

**Test cases to complete**:

  - `shared_queue_thread_pool_*`

Replace the `NaiveThreadPool` used by `KvServer` with `SharedQueueThreadPool`.
Again your `kvs-server` should still work the same as previously, now with a
slightly more clever multithreading model. This time you'll want to call
the thread pool constructor with an appropriate number of threads. For
now you can create a thread per CPU, using the [`num_cpus`] crate. We'll
revisit the number of threads later.

[`num_cpus`]: https://docs.rs/num_cpus/

