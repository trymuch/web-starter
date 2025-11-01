#![allow(unused)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DatabaseConfig {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
}

impl DatabaseConfig {
    pub(crate) fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("localhost")
    }
    pub(crate) fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }
    pub(crate) fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("postgres")
    }
    pub(crate) fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("201030")
    }
    pub(crate) fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("postgres")
    }
    pub(crate) fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}
