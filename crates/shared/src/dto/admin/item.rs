use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::validation::{
    validate_class, validate_equipment_slot, validate_inventory_type, validate_rarity,
    validate_stats, validate_uuid,
};

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct CreateItemResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct GiveItemResponse {
    pub ok: bool,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ItemSummary {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub rarity: String,
    pub inventory_type: String,
    pub class: Option<String>,
    pub equipment_slot: Option<String>,
    pub level_req: Option<i16>,
    pub max_stack: i16,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ListItemsQuery {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(range(min = 1))]
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListItemsResponse {
    pub items: Vec<ItemSummary>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
