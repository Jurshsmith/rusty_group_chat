use std::net::{SocketAddr, ToSocketAddrs};

use dotenvy::dotenv;

#[derive(Debug)]
pub struct AppServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppServerConfig {
    pub fn get() -> Self {
        dotenv().ok();

        let server_host = std::env::var("SERVER_HOST").unwrap_or(Self::default_host());
        let server_port = std::env::var("SERVER_PORT").unwrap_or(Self::default_port());

        AppServerConfig {
            host: server_host,
            port: server_port.parse().unwrap(),
        }
    }

    pub fn socket_address(&self) -> SocketAddr {
        self.host_with_port()
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    }

    fn host_with_port(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    fn default_port() -> String {
        "4003".to_string()
    }
}
