#![allow(dead_code)]
use axum::response::IntoResponse;

use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,
    #[error("Method not allowed")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("Internal servererror")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            Self::NotFound => axum::http::StatusCode::NOT_FOUND,
            Self::Biz(_) => axum::http::StatusCode::BAD_REQUEST,
            Self::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            Self::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), ApiResponse::<()>::err(self.to_string())).into_response()
    }
}
