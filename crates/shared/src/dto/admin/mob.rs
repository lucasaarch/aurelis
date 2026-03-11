use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::utils::validation::validate_mob_type;

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateMobRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    pub description: Option<String>,

    #[validate(custom(function = validate_mob_type))]
    pub mob_type: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMobResponse {
    pub id: String,
    pub slug: String,
    pub name: String,
}
