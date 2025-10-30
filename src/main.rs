use std::env;

use anyhow::Context;
use axum::{Router, routing};
use tokio::net::TcpListener;
use tracing::info;

mod configuration;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "trace");
        env::set_var("APP_CONFIG.SERVER.HOST", "localhost");
    }
    logger::init()?;

    let router = Router::new().route("/", routing::get(index));
    let app_config = configuration::get();
    let listener = TcpListener::bind(format!(
        "{}:{}",
        app_config.server.host(),
        app_config.server.port()
    ))
    .await
    .with_context(|| {
        format!(
            "Failed to bind to {}:{}",
            app_config.server.host(),
            app_config.server.port()
        )
    })?;
    info!(
        "Server is running on {}:{}",
        app_config.server.host(),
        app_config.server.port()
    );

    axum::serve(listener, router).await.with_context(|| {
        format!(
            "Failed to start server on {}:{}",
            app_config.server.host(),
            app_config.server.port()
        )
    })?;

    Ok(())
}

async fn index() -> &'static str {
    "Hello, axum!"
}
