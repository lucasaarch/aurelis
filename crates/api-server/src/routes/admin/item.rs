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
use crate::services::item::CreateItemInput;
use crate::utils::validation::{
    validate_class, validate_equipment_slot, validate_rarity, validate_stats,
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateItemRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    #[validate(custom(function = validate_class))]
    pub class: Option<String>,

    pub description: Option<String>,

    #[validate(custom(function = validate_rarity))]
    pub rarity: String,

    #[validate(custom(function = validate_equipment_slot))]
    pub equipment_slot: Option<String>,

    #[validate(range(min = 1, max = 40))]
    pub level_req: i16,

    #[validate(custom(function = validate_stats))]
    pub stats: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
pub struct CreateItemResponse {
    pub id: String,
    pub name: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/admin/items", post(create_item))
}

#[utoipa::path(
    post,
    path = "/admin/items",
    request_body = CreateItemRequest,
    responses(
        (status = 201, description = "Item created", body = CreateItemResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn create_item(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    ValidatedBody(body): ValidatedBody<CreateItemRequest>,
) -> Result<(StatusCode, Json<CreateItemResponse>), ErrorResponse> {
    let created = state
        .item_service
        .create(
            admin_id,
            CreateItemInput {
                name: body.name,
                class: body.class,
                description: body.description,
                rarity: body.rarity,
                equipment_slot: body.equipment_slot,
                level_req: body.level_req,
                stats: body.stats,
            },
        )
        .await
        .map_err(ErrorResponse::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateItemResponse {
            id: created.id.to_string(),
            name: created.name,
        }),
    ))
}
