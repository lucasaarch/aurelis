use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use crate::utils::validation::validate_inventory_type;

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GiveItemRequest {
    #[validate(length(min = 1, max = 64))]
    pub character_username: String,

    #[validate(length(min = 1, max = 64))]
    pub item_slug: String,

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
    pub inventory_type: String,
    pub created_at: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetailsResponse {
    pub id: String,
    pub slug: String,
    pub inventory_type: String,
    pub created_at: String,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_level_range"))]
pub struct ListItemsQuery {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(range(min = 1))]
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[validate(custom(function = validate_inventory_type))]
    pub inventory_type: Option<String>,

    pub search: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

fn validate_level_range(query: &ListItemsQuery) -> Result<(), ValidationError> {
    let _ = query;
    Ok(())
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
