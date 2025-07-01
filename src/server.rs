use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvsEngine, Result};

use log::{debug, error};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// A key-value store server that handles TCP connections and processes requests
/// using a given storage engine implementing the `KvsEngine` trait.
///
/// The `KvsServer` listens for incoming client connections, deserializes
/// requests, processes them through the engine, and serializes responses back
/// to the client. It supports `GET`, `SET`, and `REMOVE` operations.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Creates a new `KvsServer` instance with the provided key-value storage engine.
    pub fn new(engine: E) -> Self {
        Self { engine }
    }
    /// Starts the key-value server, binds to the given address, and handles incoming
    /// client connections.
    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    error!("Connection failed: {}", e);
                    continue;
                }
            };

            if let Err(e) = self.serve(stream) {
                error!("Error serving client: {}", e);
            }
        }
        Ok(())
    }
    /// Handles a single client connection over the given `TcpStream`.
    fn serve(&mut self, tcp: TcpStream) -> Result<()> {
        let peer_addr = tcp.peer_addr()?;
        let reader = BufReader::new(tcp.try_clone()?);
        let mut writer = BufWriter::new(tcp);
        let reqs = Deserializer::from_reader(reader).into_iter::<Request>();

        macro_rules! send_resp {
            ($resp:expr) => {{
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?;
                debug!("Response sent to {}: {:?}", peer_addr, resp);
            }};
        }

        for req in reqs {
            let req = req?;
            debug!("Receive request from {}: {:?}", peer_addr, req);
            match req {
                Request::Get { key } => send_resp!(match self.engine.get(key) {
                    Ok(value) => GetResponse::Ok(value),
                    Err(e) => GetResponse::Err(format!("{}", e)),
                }),
                Request::Set { key, value } => send_resp!(match self.engine.set(key, value) {
                    Ok(_) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(format!("{}", e)),
                }),
                Request::Remove { key } => send_resp!(match self.engine.remove(key) {
                    Ok(_) => RemoveResponse::Ok(()),
                    Err(e) => RemoveResponse::Err(format!("{}", e)),
                }),
            };
        }
        Ok(())
    }
}
