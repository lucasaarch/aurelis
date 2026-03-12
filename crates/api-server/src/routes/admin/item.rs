use crate::dto::admin::item::{
    GiveItemRequest, GiveItemResponse, ItemDetailsResponse, ItemSummary, ListItemsQuery,
    ListItemsResponse,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::models::{
    character_class::CharacterClassModel, equipment_slot::EquipmentSlotModel,
    inventory_type::InventoryTypeModel, item_rarity::ItemRarityModel,
};
use crate::repositories::item::ListItemFilters;
use crate::routes::middlewares::{AuthUser, ValidatedBody, ValidatedQuery};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/items", get(list_items))
        .route("/admin/items/{slug}", get(get_item))
        .route("/admin/items/give", post(give_item))
}

#[utoipa::path(
    get,
    path = "/admin/items",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<i64>, Query, description = "Items per page (default: 20)"),
        ("class" = Option<String>, Query, description = "Filter by class"),
        ("rarity" = Option<String>, Query, description = "Filter by rarity"),
        ("equipmentSlot" = Option<String>, Query, description = "Filter by equipment slot"),
        ("inventoryType" = Option<String>, Query, description = "Filter by inventory type"),
        ("levelMin" = Option<i16>, Query, description = "Minimum level requirement"),
        ("levelMax" = Option<i16>, Query, description = "Maximum level requirement"),
        ("search" = Option<String>, Query, description = "Search by name"),
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

    let filters = ListItemFilters {
        class: query
            .class
            .as_deref()
            .and_then(|value| value.parse::<CharacterClassModel>().ok()),
        rarity: query
            .rarity
            .as_deref()
            .and_then(|value| value.parse::<ItemRarityModel>().ok()),
        equipment_slot: query
            .equipment_slot
            .as_deref()
            .and_then(|value| value.parse::<EquipmentSlotModel>().ok()),
        inventory_type: query
            .inventory_type
            .as_deref()
            .and_then(|value| value.parse::<InventoryTypeModel>().ok()),
        level_min: query.level_min,
        level_max: query.level_max,
        search: query.search.clone(),
    };

    let (items, total) = state
        .item_service
        .list(admin_id, page, limit, filters)
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
            inventory_type: i.inventory_type.to_string(),
            class: i.class.map(|c| c.to_string()),
            equipment_slot: i.equipment_slot.map(|s| s.to_string()),
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
    get,
    path = "/admin/items/{slug}",
    params(
        ("slug" = String, Path, description = "Item slug"),
    ),
    responses(
        (status = 200, description = "Item found", body = ItemDetailsResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn get_item(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    Path(slug): Path<String>,
) -> Result<Json<ItemDetailsResponse>, ErrorResponse> {
    let item = state
        .item_service
        .get_by_slug(admin_id, slug)
        .await
        .map_err(ErrorResponse::from)?;

    Ok(Json(ItemDetailsResponse {
        id: item.id.to_string(),
        slug: item.slug,
        name: item.name,
        rarity: item.rarity.to_string(),
        inventory_type: item.inventory_type.to_string(),
        class: item.class.map(|c| c.to_string()),
        equipment_slot: item.equipment_slot.map(|s| s.to_string()),
        level_req: item.level_req,
        max_stack: item.max_stack,
        description: item.description,
        stats: item.stats,
        created_at: item.created_at.to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/admin/items/give",
    request_body = GiveItemRequest,
    responses(
        (status = 200, description = "Item given", body = GiveItemResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
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
            body.item_slug,
            body.character_username,
            body.quantity,
        )
        .await
        .map_err(ErrorResponse::from)?;

    Ok((StatusCode::OK, Json(GiveItemResponse { ok: true })))
}
