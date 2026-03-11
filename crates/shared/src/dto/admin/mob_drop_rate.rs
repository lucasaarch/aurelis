use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::validation::validate_drop_chance;

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateMobDropRateRequest {
    #[schema(value_type = String, format = "uuid")]
    pub mob_id: Uuid,

    #[schema(value_type = String, format = "uuid")]
    pub item_id: Uuid,

    #[validate(custom(function = validate_drop_chance))]
    #[schema(value_type = f64, format = "decimal")]
    pub drop_chance: BigDecimal,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMobDropRateResponse {
    pub id: String,
}
