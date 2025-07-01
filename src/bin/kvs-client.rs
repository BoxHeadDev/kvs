use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};
use std::net::SocketAddr;
use std::process;

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
enum Command {
    /// Get the string value of a given string key
    Get {
        /// A string key
        key: String,

        /// Server address
        #[arg(long, default_value = DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    /// Set the value of a string key to a string
    Set {
        /// A string key
        key: String,

        /// The string value of the key
        value: String,

        /// Server address
        #[arg(long, default_value = DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    /// Remove a given string key
    Rm {
        /// A string key
        key: String,

        /// Server address
        #[arg(long, default_value = DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
}

fn main() {
    let opt = Opt::parse();

    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Get { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            match client.get(key)? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Command::Set { key, value, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.set(key, value)?;
        }
        Command::Rm { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.remove(key)?;
        }
    }
    Ok(())
}
