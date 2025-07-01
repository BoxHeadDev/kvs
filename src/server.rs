use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvStore, Result};
use log::{debug, error};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct KvsServer {
    engine: KvStore,
}

impl KvsServer {
    pub fn run<A: ToSocketAddrs>(self, addr: A) -> Result<()> {
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

    fn serve(&self, tcp: TcpStream) -> Result<()> {
        let peer_addr = tcp.peer_addr()?;
        let reader = BufReader::new(tcp.try_clone()?);
        let mut writer = BufWriter::new(tcp);
        let reqs = Deserializer::from_reader(reader).into_iter::<Request>();

        for req in reqs {
            let req = req?;
            debug!("Request from {}: {:?}", peer_addr, req);
            let resp = match req {
                Request::Get { key } => self
                    .engine
                    .get(key)
                    .map(GetResponse::Ok)
                    .unwrap_or_else(|e| GetResponse::Err(e.to_string())),
                Request::Set { key, value } => self
                    .engine
                    .set(key, value)
                    .map(|_| SetResponse::Ok(()))
                    .unwrap_or_else(|e| SetResponse::Err(e.to_string())),
                Request::Remove { key } => self
                    .engine
                    .remove(key)
                    .map(|_| RemoveResponse::Ok(()))
                    .unwrap_or_else(|e| RemoveResponse::Err(e.to_string())),
            };
            serde_json::to_writer(&mut writer, &resp)?;
            writer.flush()?;
            debug!("Response sent to {}: {:?}", peer_addr, resp);
        }

        Ok(())
    }
}
