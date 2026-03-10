use std::sync::Arc;

use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::app::AppState;
use crate::error::{AppError, ErrorResponse};
use crate::services::account::RegisterParams;

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/auth/register", post(register))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Account created"),
        (status = 409, description = "Email or username already in use", body = ErrorResponse),
    ),
    tag = "Auth",
)]
pub async fn register(
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
