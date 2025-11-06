use std::time::Duration;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{self, CorsLayer},
    normalize_path::NormalizePathLayer,
};

use crate::{app::AppState, configuration::ServerConfig, latency};

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
        let timeout_layer = tower_http::timeout::TimeoutLayer::new(Duration::from_secs(2 * 60));
        let body_limit_layer = DefaultBodyLimit::max(bytesize::mib(10_u64) as usize);
        let cors_layer = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));
        let normalize_path_layer = NormalizePathLayer::trim_trailing_slash();
        let tracing_layer = tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(|req: &Request| {
                let method = req.method();
                let path = req.uri().path();
                let id = xid::new();
                tracing::info_span!("API Request",id = %id,method = %method,path = %path)
            })
            .on_request(())
            .on_failure(())
            // .on_response(
            //     DefaultOnResponse::new()
            //         .level(tracing::Level::INFO)
            //         .latency_unit(tower_http::LatencyUnit::Micros),
            // )
            .on_response(latency::LatencyOnResponse);

        Router::new()
            .merge(router)
            .layer(timeout_layer)
            .layer(body_limit_layer)
            .layer(tracing_layer)
            .layer(cors_layer)
            .layer(normalize_path_layer)
            .with_state(state)
    }
}
