use clap::{Parser, ValueEnum};
use kvs::thread_pool::*;
use kvs::*;
use log::{LevelFilter, error, info, warn};
use std::env::current_dir;
use std::fmt;
use std::fs;
use std::net::SocketAddr;
use std::process;
use std::str::FromStr;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(Parser, Debug)]
#[command(name = "kvs-server")]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Opt {
    /// Sets the listening address
    #[arg(long, value_name = "IP:PORT", default_value = DEFAULT_LISTENING_ADDRESS)]
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

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Engine::kvs => "kvs",
            Engine::sled => "sled",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Engine {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "kvs" => Ok(Engine::kvs),
            "sled" => Ok(Engine::sled),
            _ => Err(format!("Unknown engine: {}", s)),
        }
    }
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
    let current = current_engine()?;
    let selected = opt.engine.unwrap_or(DEFAULT_ENGINE);

    if let Some(existing) = current {
        if existing != selected {
            error!(
                "Engine mismatch: previously used '{}', but '{}' was requested",
                existing, selected
            );
            process::exit(1);
        }
    }

    run(opt, selected)
}

fn run(opt: Opt, engine: Engine) -> Result<()> {
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", engine);
    info!("Listening on {}", opt.addr);

    fs::write(current_dir()?.join("engine"), engine.to_string())?;

    let pool = RayonThreadPool::new(num_cpus::get() as u32)?;

    match engine {
        Engine::kvs => {
            let store = KvStore::open(current_dir()?)?;
            KvsServer::new(store, pool).run(opt.addr)
        }
        Engine::sled => {
            let store = SledKvsEngine::open(current_dir()?)?;
            KvsServer::new(store, pool).run(opt.addr)
        }
    }
}

fn current_engine() -> Result<Option<Engine>> {
    let path = current_dir()?.join("engine");
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)?;
    match content.trim().parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            warn!("Invalid engine file '{}': {}", path.display(), e);
            Ok(None)
        }
    }
}
