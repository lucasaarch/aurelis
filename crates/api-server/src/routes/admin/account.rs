use crate::dto::admin::account::{
    AccountSummary, ListAccountsQuery, ListAccountsResponse, PunishAccountRequest,
    PunishAccountResponse,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::repositories::account::ListAccountFilters;
use crate::routes::middlewares::{AuthUser, ValidatedBody, ValidatedQuery};
use crate::utils::parsers::parse_suspension_severity;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/accounts", get(list_accounts))
        .route("/admin/accounts/{id}/punishments", post(punish_account))
}

#[utoipa::path(
    get,
    path = "/admin/accounts",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<i64>, Query, description = "Accounts per page (default: 20)"),
        ("search" = Option<String>, Query, description = "Search by email"),
    ),
    responses(
        (status = 200, description = "Accounts listed", body = ListAccountsResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn list_accounts(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    ValidatedQuery(query): ValidatedQuery<ListAccountsQuery>,
) -> Result<Json<ListAccountsResponse>, ErrorResponse> {
    let page = query.page.max(1);
    let limit = query.limit.clamp(1, 100);

    let filters = ListAccountFilters {
        search: query.search.clone(),
    };

    let (accounts, total) = state
        .account_service
        .list(admin_id, page, limit, filters)
        .await
        .map_err(ErrorResponse::from)?;

    let total_pages = (total + limit - 1) / limit;
    let accounts = accounts
        .into_iter()
        .map(|a| AccountSummary {
            id: a.id.to_string(),
            email: a.email,
            is_admin: a.is_admin,
            created_at: a.created_at.to_string(),
        })
        .collect();

    Ok(Json(ListAccountsResponse {
        accounts,
        total,
        page,
        limit,
        total_pages,
    }))
}

#[utoipa::path(
    post,
    path = "/admin/accounts/{id}/punishments",
    params(
        ("id" = String, Path, description = "Account id"),
    ),
    request_body = PunishAccountRequest,
    responses(
        (status = 200, description = "Account updated", body = PunishAccountResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn punish_account(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    axum::extract::Path(account_id): axum::extract::Path<uuid::Uuid>,
    ValidatedBody(body): ValidatedBody<PunishAccountRequest>,
) -> Result<(StatusCode, Json<PunishAccountResponse>), ErrorResponse> {
    let punishment_type = body.punishment_type.as_str().to_string();
    let severity = match punishment_type.as_str() {
        "suspend" => Some(parse_suspension_severity(
            body.severity.as_deref().unwrap_or("medium"),
        )?),
        _ => None,
    };

    let updated = state
        .account_service
        .apply_punishment(
            admin_id,
            account_id,
            body.punishment_type,
            body.reason,
            severity,
        )
        .await?;

    Ok((
        StatusCode::OK,
        Json(PunishAccountResponse {
            id: updated.id.to_string(),
            punishment_type,
            banned_at: updated.banned_at.map(|v| v.to_string()),
            banned_reason: updated.banned_reason,
            suspended_until: updated.suspended_until.map(|v| v.to_string()),
        }),
    ))
}
