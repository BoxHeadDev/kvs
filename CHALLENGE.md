#  Challenge

## 🚀 Challenge: Multithreded KvServer

Let's quickly review our architecture here: `KvServer` sets up a TCP socket and
begins listening on it; when it receives a request it deserializes it and calls
some implementation of the `KvsEngine` trait to store or retrieve data; then it
sends back the response. The details of how `KvsEngine` works don't matter to
`KvServer`.

So in the last project you probably created a loop vaguelly like:

```rust
let listener = TcpListener::bind(addr)?;

for stream in listener.incoming() {
	let cmd = self.read_cmd(&stream);
	let resp = self.process_cmd(cmd);
	self.respond(&stream, resp);
}
```

_Well, now you just need to do the same thing, but spawn all the work inside the
loop into your `NaiveThreadPool`_. The database query and the response are both
handled on a different thread than the TCP listener. This offloads most of
the hard work to other threads, allowing the recieving thread to process more
requests. It should increase throughput, at least on multi-core machines.

Again, you should still have a working client/server key-value store, now
multithreaded.
