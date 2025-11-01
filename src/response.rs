use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u16, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
    pub fn ok(msg: impl AsRef<str>, data: Option<T>) -> Self {
        Self::new(0, msg.as_ref().to_string(), data)
    }
    pub fn err(msg: impl AsRef<str>) -> Self {
        Self::new(1, msg.as_ref().to_string(), None)
    }
}

impl<T> IntoResponse for ApiResponse<T>
where

    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
