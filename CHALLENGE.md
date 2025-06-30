# Buffered I/O Wrappers Challenge

💡 Challenge: Implement Buffered I/O Wrappers with Position Tracking

Create two custom structs in Rust:
- `BufReaderWithPos`
- `BufWriterWithPos`

Each should wrap `BufReader` and `BufWriter` respectively, while keeping track of the current seek position (`pos`) of the underlying reader or writer.

### Requirements:
1. Both structs should be generic over any type that implements `Read + Seek` (for reader) or `Write + Seek` (for writer).
2. Implement the `Read` and `Seek` traits for `BufReaderWithPos`.
3. Implement the `Write`, `Flush`, and `Seek` traits for `BufWriterWithPos`.
4. In each trait implementation, ensure the `pos` field is accurately updated after operations.
5. Each struct should have a constructor (`new`) that initializes the wrapper and position correctly.

💡 Hint: Use `SeekFrom::Current(0)` to get the initial position.
💡 Use `BufReader` and `BufWriter` from `std::io`, and wrap the given reader/writer in them.

📁 File structure:
```rust
// log_io.rs

use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use crate::Result;

// implement BufReaderWithPos and BufWriterWithPos here
```

This challenge tests your ability to work with generics, traits, and low-level I/O abstractions in Rust. Good luck!
