use std::io;

use anyhow::Context;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn init() -> anyhow::Result<()> {
    // let log_file = tracing_appender::rolling::daily("./log", "web-starter");
    let log_file = tracing_appender::rolling::RollingFileAppender::builder()
        .max_log_files(30)
        .filename_prefix("web-starter")
        .filename_suffix("log")
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .build("./logs")
        .with_context(|| "Failed to create log file")?;

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .expect("Failed to parse environment variable RUST_LOG"),
        )
        .with(
            fmt::layer()
                .with_timer(fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".into()))
                .with_target(false)
                .with_thread_names(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_writer(io::stdout),
        )
        .with(
            fmt::layer()
                .with_timer(fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S.3f".into()))
                .with_ansi(false)
                .with_thread_names(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_writer(log_file),
        )
        .init();
    Ok(())
}
