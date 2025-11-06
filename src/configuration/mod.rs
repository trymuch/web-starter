#![allow(unused)]

use anyhow::Context;
use serde::Deserialize;
use std::sync::LazyLock;

pub(crate) use database::DatabaseConfig;
pub(crate) use server::ServerConfig;

mod database;
mod server;
#[derive(Debug, Deserialize)]
pub(crate) struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

impl AppConfig {
    pub(crate) fn load() -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(
                config::File::with_name("application").required(false), // .format(config::FileFormat::Toml),
            )
            .add_source(
                config::Environment::with_prefix("APP_CONFIG")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true)
                    .convert_case(config::Case::Snake),
            )
            .build()
            .with_context(|| "Failed to load configuration")?;

        let app_config = settings
            .try_deserialize()
            .with_context(|| "Failed to deserialize configuration")?;

        tracing::info!("application configuration loaded");

        Ok(app_config)
    }

    pub(crate) fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub(crate) fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| { 
    AppConfig::load().expect("Failed to initialize configuration")
});

pub(crate) fn get() -> &'static AppConfig {
    &APP_CONFIG
}
