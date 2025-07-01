use clap::{Parser, ValueEnum};
use std::net::SocketAddr;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(Parser, Debug)]
#[command(name = "kvs-server")]
struct Opt {
    /// Sets the listening address
    #[arg(long, value_name = "IP:PORT", default_value = DEFAULT_LISTENING_ADDRESS)]
    addr: SocketAddr,

    /// Sets the storage engine
    #[arg(long, value_name = "ENGINE-NAME", default_value = DEFAULT_ENGINE)]
    engine: Option<Engine>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
#[allow(non_camel_case_types)]
enum Engine {
    kvs,
    sled,
}

fn main() {
    let mut opt = Opt::parse();
}
