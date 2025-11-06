use axum::Router;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
};

mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/users", user::create_router())
                .fallback(async || -> ApiResult<()> {
                    tracing::warn!("请求的资源Not found");
                    Err(ApiError::NotFound)
                }),
        )
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method not allowed");
            Err(ApiError::MethodNotAllowed)
        })
}
