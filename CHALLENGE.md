#  Challenge

## 🚀 Challenge: Multithreading

Your first try at introducing concurrency is going to be the simplest: spawning a new thread per incoming connection, and responding to the request on that connection, then letting the thread exit. What performance benefits will distributing work across threads provide? How do you expect latency will be affected? What about throughput?

The first step is to write a ThreadPool implementation for this naive approach, where ThreadPool::spawn will create a new thread for each spawned job. Call it NaiveThreadPool (it's not really even a thread pool since this implementation is not going to reuse threads between jobs, but it needs to conform to our trait for later comparisons).

We aren't focusing on a more sophisticated implementation now because simply integrating this solution into our existing design is going to take some effort. Note that the ThreadPool::new constructor takes a threads argument specifying the number of threads in the pool. In this implementation it will be unused.

Go ahead and implement this version of ThreadPool now, then we'll integrate it into the new KvStore.
