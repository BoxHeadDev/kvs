#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use index::CommandPos;
pub use kv::KvStore;
pub use log_io::{BufReaderWithPos, BufWriterWithPos};

mod error;
mod index;
mod kv;
mod log_io;
