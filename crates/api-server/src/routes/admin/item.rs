use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody};
use crate::services::item::CreateItemInput;
use shared::utils::validation::{
    validate_class, validate_equipment_slot, validate_inventory_type, validate_rarity,
    validate_stats, validate_uuid,
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
    pub level_req: Option<i16>,

    #[validate(custom(function = validate_stats))]
    pub stats: Option<serde_json::Value>,

    #[validate(custom(function = validate_inventory_type))]
    pub inventory_type: String,

    #[validate(range(min = 1))]
    pub max_stack: Option<i16>,
}

#[derive(Serialize, ToSchema)]
pub struct CreateItemResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct GiveItemRequest {
    #[validate(custom(function = validate_uuid))]
    #[schema(value_type = String, format = "uuid")]
    pub character_id: Uuid,

    #[validate(custom(function = validate_uuid))]
    #[schema(value_type = String, format = "uuid")]
    pub item_id: Uuid,

    #[validate(range(min = 1))]
    pub quantity: Option<i16>,
}

#[derive(Serialize, ToSchema)]
pub struct GiveItemResponse {
    pub ok: bool,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/items", post(create_item))
        .route("/admin/items/give", post(give_item))
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
                inventory_type: body.inventory_type,
                max_stack: body.max_stack,
            },
        )
        .await
        .map_err(ErrorResponse::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateItemResponse {
            id: created.id.to_string(),
            name: created.name,
            slug: created.slug,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/admin/items/give",
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
    state
        .item_service
        .give_item(
            admin_id,
            body.character_id,
            body.item_id,
            body.quantity.unwrap_or(1),
        )
        .await
        .map_err(ErrorResponse::from)?;

    Ok((StatusCode::CREATED, Json(GiveItemResponse { ok: true })))
}
