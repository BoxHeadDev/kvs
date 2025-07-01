use crate::Result;
use log::error;
use std::net::{TcpListener, ToSocketAddrs};

pub struct KvsServer {}

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
        }
        Ok(())
    }
}
