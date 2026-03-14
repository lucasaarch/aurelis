use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::routes;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().unwrap().add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::admin::item::list_items,
        routes::admin::item::get_item,
        routes::admin::item::give_item,
        routes::auth::register,
        routes::auth::login,
        routes::auth::refresh_token,
    ),
    tags(
        (name = "Admin", description = "Admin-only endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
    ),
    components(
        schemas(
            crate::dto::admin::account::ListAccountsQuery,
            crate::dto::admin::account::AccountSummary,
            crate::dto::admin::account::ListAccountsResponse,
            crate::dto::admin::account::PunishAccountRequest,
            crate::dto::admin::account::PunishAccountResponse,
            crate::dto::admin::item::ListItemsQuery,
            crate::dto::admin::item::ItemDetailsResponse,
            crate::dto::admin::item::GiveItemRequest,
            crate::dto::admin::item::GiveItemResponse,
        )
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Resona API"
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api.resona.dev", description = "Production server")
    )
)]
pub struct ApiDoc;
