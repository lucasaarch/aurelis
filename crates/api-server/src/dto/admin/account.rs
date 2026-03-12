use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use crate::utils::validation::{validate_punishment_type, validate_suspension_severity};

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsQuery {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(range(min = 1))]
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[validate(length(min = 1, max = 255))]
    pub search: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    pub id: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountSummary>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_punishment_request"))]
pub struct PunishAccountRequest {
    #[validate(custom(function = validate_punishment_type))]
    pub punishment_type: String,

    #[validate(length(min = 1, max = 255))]
    pub reason: Option<String>,

    #[validate(custom(function = validate_suspension_severity))]
    pub severity: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PunishAccountResponse {
    pub id: String,
    pub punishment_type: String,
    pub banned_at: Option<String>,
    pub banned_reason: Option<String>,
    pub suspended_until: Option<String>,
}

fn validate_punishment_request(request: &PunishAccountRequest) -> Result<(), ValidationError> {
    if request.punishment_type == "ban" && request.reason.is_none() {
        return Err(ValidationError::new("ban_reason_required"));
    }

    if request.punishment_type == "suspend" && request.severity.is_none() {
        return Err(ValidationError::new("suspension_severity_required"));
    }

    Ok(())
}
