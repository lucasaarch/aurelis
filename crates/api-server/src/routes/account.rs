use std::sync::Arc;

use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;

use crate::app::AppState;
use crate::error::AppError;
use crate::services::account::RegisterParams;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/auth/register", post(register))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    state
        .account_service
        .register(RegisterParams {
            username: body.username,
            email: body.email,
            password: body.password,
        })
        .await?;

    Ok(StatusCode::CREATED)
}
