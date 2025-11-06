mod api;
mod app;
mod common;
mod configuration;
mod database;
mod entity;
mod error;
mod latency;
mod logger;
mod response;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "DEBUG");
    }

    app::run(api::create_router()).await
}
