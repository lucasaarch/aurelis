use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody};
use crate::services::mob::CreateMobInput;
use crate::utils::validation::validate_mob_type;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateMobRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    pub description: Option<String>,

    #[validate(custom(function = validate_mob_type))]
    pub mob_type: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateMobResponse {
    pub id: String,
    pub slug: String,
    pub name: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/admin/mobs", post(create_mob))
}

#[utoipa::path(
    post,
    path = "/admin/mobs",
    request_body = CreateMobRequest,
    responses(
        (status = 201, description = "Mob created", body = CreateMobResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin"
)]
pub async fn create_mob(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_id): AuthUser,
    ValidatedBody(body): ValidatedBody<CreateMobRequest>,
) -> Result<(StatusCode, Json<CreateMobResponse>), ErrorResponse> {
    let created = state
        .mob_service
        .create(
            admin_id,
            CreateMobInput {
                name: body.name,
                description: body.description,
                mob_type: body.mob_type,
            },
        )
        .await
        .map_err(ErrorResponse::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateMobResponse {
            id: created.id.to_string(),
            slug: created.slug,
            name: created.name,
        }),
    ))
}
