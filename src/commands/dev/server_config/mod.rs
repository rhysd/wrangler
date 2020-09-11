mod host;
mod protocol;

pub use protocol::Protocol;

use host::Host;

use std::net::{SocketAddr, TcpListener};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: Host,
    pub listening_address: SocketAddr,
}

impl ServerConfig {
    pub fn new(
        host: Option<&str>,
        ip: Option<&str>,
        port: Option<u16>,
        upstream_protocol: Protocol,
    ) -> Result<Self, failure::Error> {
        let ip = ip.unwrap_or("127.0.0.1");
        let port = port.unwrap_or(8787);
        let addr = format!("{}:{}", ip, port);
        let listening_address = match TcpListener::bind(&addr) {
            Ok(socket) => socket.local_addr(),
            Err(_) => failure::bail!("{} is unavailable, try binding to another address with the --port and --ip flags, or stop other `wrangler dev` processes.", &addr)
        }?;

        let host = if let Some(host) = host {
            Host::new(&host, false)?
        } else {
            Host::new(
                match upstream_protocol {
                    Protocol::Http => "http://tutorial.cloudflareworkers.com",
                    Protocol::Https => "https://tutorial.cloudflareworkers.com",
                },
                true,
            )?
        };

        Ok(ServerConfig {
            host,
            listening_address,
        })
    }
}
