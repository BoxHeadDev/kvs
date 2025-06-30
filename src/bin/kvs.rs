use std::process::exit;
use structopt::StructOpt;

/// A simple key-value store
#[derive(StructOpt, Debug)]
#[structopt(name = "kvs")]
pub struct Config {
    /// Subcommand to run
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Set the value of a key
    Set { key: String, value: String },
    /// Get the value of a key
    Get { key: String },
    /// Remove a key
    Rm { key: String },
}

fn main() {
    let config = Config::from_args();

    match config.cmd {
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
}
