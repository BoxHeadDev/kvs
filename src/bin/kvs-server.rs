use clap::{Parser, ValueEnum};
use kvs::*;
use log::{LevelFilter, error, info, warn};
use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;
use std::process;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(Parser, Debug)]
#[command(name = "kvs-server")]
struct Opt {
    /// Sets the listening address
    #[arg(long, default_value = DEFAULT_LISTENING_ADDRESS)]
    addr: SocketAddr,

    /// Sets the storage engine
    #[arg(long, value_enum)]
    engine: Option<Engine>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
#[allow(non_camel_case_types)]
enum Engine {
    kvs,
    sled,
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let opt = Opt::parse();
    if let Err(e) = try_main(opt) {
        error!("{}", e);
        process::exit(1);
    }
}

fn try_main(opt: Opt) -> Result<()> {
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Using KvStore engine");
    info!("Listening on {}", opt.addr);

    let store = KvStore::open(current_dir()?)?;
    let server = KvsServer::new(store);
    server.run(opt.addr)
}
