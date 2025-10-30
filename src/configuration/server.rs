#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ServerConfig {
    host: String,
    port: Option<u16>,
}

impl ServerConfig {
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
