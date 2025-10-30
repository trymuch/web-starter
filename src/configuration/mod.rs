mod server;

use std::sync::LazyLock;

use anyhow::Context;
use serde::Deserialize;
pub(crate) use server::ServerConfig;

#[derive(Debug, Deserialize)]
pub(crate) struct AppConfig {
    pub(crate) server: ServerConfig,
}

impl AppConfig {
    pub(crate) fn load() -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("APP_CONFIG")
                    .prefix_separator("_")
                    .separator(".")
                    .try_parsing(true)
                    // .list_separator(","),
            )
            .add_source(
                config::File::with_name("application").required(true), // .format(config::FileFormat::Toml),
            )
            .build()
            .with_context(|| "Failed to load configuration")?;
        settings
            .try_deserialize()
            .with_context(|| "Failed to deserialize configuration")
    }
}

static APP_CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initialize configuration"));

pub(crate) fn get() -> &'static AppConfig {
    &APP_CONFIG
}
