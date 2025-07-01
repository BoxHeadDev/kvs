use clap::{Parser, Subcommand};
use kvs::{KvsClient, Result};
use std::net::SocketAddr;
use std::process;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

/// A simple key-value store
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Get the string value of a given string key
    Get {
        /// A string key
        key: String,

        /// Server address
        #[arg(long, value_name = ADDRESS_FORMAT, default_value = DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    /// Set the value of a string key to a string
    Set {
        /// A string key
        key: String,

        /// The string value of the key
        value: String,

        /// Server address
        #[arg(long, value_name = ADDRESS_FORMAT, default_value = DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    /// Remove a given string key
    Rm {
        /// A string key
        key: String,

        /// Server address
        #[arg(long, value_name = ADDRESS_FORMAT, default_value = DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
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
