#![deny(missing_docs)]
//! A simple key/value store.

pub use client::KvsClient;
pub use command::Command;
pub use error::{KvsError, Result};
pub use index::CommandPos;
pub use kv::KvStore;
pub use log_io::{BufReaderWithPos, BufWriterWithPos};
pub use server::KvServer;

mod client;
mod command;
mod common;
mod error;
mod index;
mod kv;
mod log_io;
mod server;
