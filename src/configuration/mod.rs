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

        tracing::info!("configuration loaded from file source and environment variables");

        let app_config = settings
            .try_deserialize()
            .with_context(|| "Failed to deserialize configuration")?;

        tracing::info!("application configuration parsed");

        Ok(app_config)
    }
}

static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    let app_config = AppConfig::load().expect("Failed to initialize configuration");
    tracing::info!("application configuration initialized");
    app_config
});

pub(crate) fn get() -> &'static AppConfig {
    &APP_CONFIG
}
