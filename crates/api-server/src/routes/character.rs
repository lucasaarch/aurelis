use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use shared::models::character_class::CharacterClass;
use utoipa::ToSchema;
use validator::Validate;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody};
use crate::services::character::CreateCharacterInput;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateCharacterRequest {
    #[validate(length(min = 3, max = 24, message = "must be between 3 and 24 characters"))]
    pub name: String,
    pub class: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateCharacterResponse {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/character", post(create_character))
}

#[utoipa::path(
    post,
    path = "/character",
    summary = "Create a character",
    description = "Creates a new character for the authenticated account.",
    request_body = CreateCharacterRequest,
    responses(
        (status = 201, description = "Character created", body = CreateCharacterResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 409, description = "Name already taken", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Character"
)]
pub async fn create_character(
    State(state): State<Arc<AppState>>,
    AuthUser(account_id): AuthUser,
    ValidatedBody(body): ValidatedBody<CreateCharacterRequest>,
) -> Result<(StatusCode, Json<CreateCharacterResponse>), ErrorResponse> {
    let character = state
        .character_service
        .create(account_id, CreateCharacterInput {
            name: body.name,
            class: body.class,
        })
        .await
        .map_err(ErrorResponse::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateCharacterResponse {
            id: character.id.to_string(),
            name: character.name,
            class: character.class,
        }),
    ))
}
