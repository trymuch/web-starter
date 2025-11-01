use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{configuration, database, logger, server};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db_conn: DatabaseConnection,
}

impl AppState {
    pub(crate) fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

pub(crate) async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init()?;
    tracing::info!("Starting server...");
    let app_config = configuration::get();
    let db_conn = database::init(app_config.database()).await?;
    let state = AppState::new(db_conn);
    let server = server::Server::new(app_config.server());
    server.start(state, router).await?;
    Ok(())
}
