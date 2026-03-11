use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use shared::dto::admin::item::{
    CreateItemRequest, CreateItemResponse, GiveItemRequest, GiveItemResponse, ItemSummary,
    ListItemsQuery, ListItemsResponse,
};
use std::sync::Arc;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody, ValidatedQuery};
use crate::services::item::CreateItemInput;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/items", get(list_items).post(create_item))
        .route("/admin/items/give", post(give_item))
}

#[utoipa::path(
    get,
    path = "/admin/items",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<i64>, Query, description = "Items per page (default: 20)"),
    ),
    responses(
        (status = 200, description = "Items listed", body = ListItemsResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn list_items(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    ValidatedQuery(query): ValidatedQuery<ListItemsQuery>,
) -> Result<Json<ListItemsResponse>, ErrorResponse> {
    let page = query.page.max(1);
    let limit = query.limit.clamp(1, 100);

    let (items, total) = state
        .item_service
        .list(admin_id, page, limit)
        .await
        .map_err(ErrorResponse::from)?;

    let total_pages = (total + limit - 1) / limit;
    let items = items
        .into_iter()
        .map(|i| ItemSummary {
            id: i.id.to_string(),
            slug: i.slug,
            name: i.name,
            rarity: i.rarity.to_string(),
            inventory_type: i.inventory_type.into(),
            class: i.class.map(|c| c.to_string()),
            equipment_slot: i.equipment_slot.map(Into::into),
            level_req: i.level_req,
            max_stack: i.max_stack,
            description: i.description,
            created_at: i.created_at.to_string(),
        })
        .collect();

    Ok(Json(ListItemsResponse {
        items,
        total,
        page,
        limit,
        total_pages,
    }))
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
