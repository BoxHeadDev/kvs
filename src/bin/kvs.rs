use clap::{Parser, Subcommand};
use kvs::Result;
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

    match cli.command {
        Command::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
    Ok(())
}
