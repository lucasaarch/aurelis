use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody};
use shared::utils::validation::validate_drop_chance;

#[derive(Deserialize, ToSchema, Validate)]
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
pub struct CreateMobDropRateResponse {
    pub id: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/admin/mob-drop-rates", post(create_mob_drop_rate))
}

#[utoipa::path(
    post,
    path = "/admin/mob-drop-rates",
    request_body = CreateMobDropRateRequest,
    responses(
        (status = 201, description = "Mob drop rate created", body = CreateMobDropRateResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn create_mob_drop_rate(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    ValidatedBody(body): ValidatedBody<CreateMobDropRateRequest>,
) -> Result<(StatusCode, Json<CreateMobDropRateResponse>), ErrorResponse> {
    let created = state
        .mob_drop_rate_service
        .create(admin_id, body.mob_id, body.item_id, body.drop_chance)
        .await
        .map_err(ErrorResponse::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateMobDropRateResponse {
            id: created.id.to_string(),
        }),
    ))
}
