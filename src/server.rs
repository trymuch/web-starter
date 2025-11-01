use axum::Router;
use tokio::net::TcpListener;

use crate::{app::AppState, configuration::ServerConfig};

pub(crate) struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub(crate) fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub(crate) async fn start(
        &self,
        state: AppState,
        router: Router<AppState>,
    ) -> anyhow::Result<()> {
        let router = self.build_router(state, router);
        let addr = format!("{}:{}", self.config.host(), self.config.port());
        let listener = TcpListener::bind(addr).await?;
        tracing::info!("Server is running on {}", listener.local_addr()?);
        axum::serve(listener, router).await?;
        Ok(())
    }

    pub(crate) fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        Router::new().merge(router).with_state(state)
    }
}
