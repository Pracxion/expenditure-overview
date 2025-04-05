use std::{net::SocketAddr,
          str::FromStr};

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    server_host:      String,
    server_port:      u16,
    pub database_url: String
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Failed to read config")
    }
}

impl Config {
    pub fn server_address(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.server_host, self.server_port)).expect("Failed to parse address")
    }
}
