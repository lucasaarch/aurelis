use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use validator::Validate;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody};
use uuid::Uuid;

#[derive(Deserialize, ToSchema, Validate)]
pub struct GiveItemRequest {
    pub character_id: String,
    pub item_id: String,
    pub quantity: Option<i16>,
}

#[derive(Serialize, ToSchema)]
pub struct GiveItemResponse {
    pub ok: bool,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/admin/give-item", post(give_item))
}

#[utoipa::path(
    post,
    path = "/admin/give-item",
    request_body = GiveItemRequest,
    responses(
        (status = 201, description = "Item given", body = GiveItemResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn give_item(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    ValidatedBody(body): ValidatedBody<GiveItemRequest>,
) -> Result<(StatusCode, Json<GiveItemResponse>), ErrorResponse> {
    let char_id = match Uuid::parse_str(&body.character_id) {
        Ok(u) => u,
        Err(_) => return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, "INVALID_CHARACTER_ID", "Invalid character_id")),
    };

    let item_id = match Uuid::parse_str(&body.item_id) {
        Ok(u) => u,
        Err(_) => return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, "INVALID_ITEM_ID", "Invalid item_id")),
    };

    let qty = body.quantity.unwrap_or(1);

    state
        .item_service
        .give_item(admin_id, char_id, item_id, qty)
        .await
        .map_err(ErrorResponse::from)?;

    Ok((StatusCode::CREATED, Json(GiveItemResponse { ok: true })))
}
