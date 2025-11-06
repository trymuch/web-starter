use axum::{Router, extract::State, routing};

use crate::entity::prelude::*;
use crate::error::{ApiError, ApiResult};
use crate::response::ApiResponse;
use crate::{app::AppState, entity::sys_user};
use sea_orm::{Condition, prelude::*};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(get_users))
}

async fn get_users(
    State(AppState { db_conn }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("male"))
                .add(sys_user::Column::Name.starts_with("张"))
                .add(Condition::any().add(sys_user::Column::Enabled.eq(true))),
        )
        .all(&db_conn)
        .await
        .map_err(|e| ApiError::Internal(e.into()))?;
    Ok(ApiResponse::ok("OK", Some(users)))
}

pub struct UserQueryParams{
    keyword:Option<String>,
    page:Option<u64>,
    size:Option<u64>,
}