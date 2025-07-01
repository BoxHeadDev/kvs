use clap::{Parser, Subcommand};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

/// A simple key-value store
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Set the value of a key
    Set { key: String, value: String },
    /// Get the value of a key
    Get { key: String },
    /// Remove a key
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut store = KvStore::open(current_dir()?)?
    
    match cli.command {
        Command::Set { key, value } => {
            store.set(key.to_string(), value.to_string())?;
        }
        Command::Get { key } => match store.get(key)? {
            Some(value) => println!("{}", value),
            None => println!("Key not found"),
        },
        Command::Rm { key } => match store.remove(key) {
            Ok(()) => {}
            Err(KvsError::KeyNotFound) => {
                println!("Key not found");
                exit(1);
            }
            Err(e) => return Err(e),
        },
    }
    Ok(())
}
