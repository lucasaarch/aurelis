use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::info;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};

use http::header::HeaderValue;
use http::{Method, header};
use tower_http::cors::CorsLayer;

use crate::config::Config;
use crate::db::Database;
use crate::grpc::auth::GrpcAuthServiceImpl;
use crate::grpc::character::GrpcCharacterServiceImpl;
use crate::grpc::inventory::GrpcInventoryServiceImpl;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::PgCharacterRepository;
use crate::repositories::inventory::PgInventoryRepository;
use crate::repositories::item::PgItemRepository;
use crate::routes;
use crate::services::account::AccountService;
use crate::services::auth::AuthService;
use crate::services::character::CharacterService;
use crate::services::hash::HashService;
use crate::services::inventory::InventoryService;
use crate::services::item::ItemService;
use crate::services::jwt::JwtService;
use shared::proto::auth::auth_service_server::AuthServiceServer;
use shared::proto::character::character_service_server::CharacterServiceServer;
use shared::proto::inventory::inventory_service_server::InventoryServiceServer;

struct SecurityAddon;

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
            // mob schemas removed
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

#[derive(Clone)]
pub struct AppState {
    pub account_service: AccountService,
    pub auth_service: AuthService,
    pub character_service: CharacterService,
    pub jwt_service: JwtService,
    pub item_service: ItemService,
    pub inventory_service: InventoryService,
}

impl AppState {
    fn new(db: Database, config: &Config) -> Self {
        let account_repository = PgAccountRepository::new(db.clone());
        let character_repository = PgCharacterRepository::new(db.clone());
        let item_repository = PgItemRepository::new(db.clone());
        let inventory_repository = PgInventoryRepository::new(db.clone());

        let hash_service = HashService::default();
        let jwt_service = JwtService::new(
            &config.jwt.secret_web,
            &config.jwt.secret_game,
            &config.jwt.refresh_secret_web,
            &config.jwt.refresh_secret_game,
            config.jwt.expiration_seconds,
            config.jwt.refresh_expiration_seconds,
        );

        let account_service = AccountService::new(account_repository.clone());
        let auth_service = AuthService::new(
            account_repository.clone(),
            hash_service.clone(),
            jwt_service.clone(),
        );
        let character_service =
            CharacterService::new(character_repository, account_repository.clone());
        let inventory_service = InventoryService::new(inventory_repository.clone());
        let item_service = ItemService::new(
            item_repository,
            account_service.clone(),
            character_service.clone(),
            inventory_service.clone(),
        );

        Self {
            account_service,
            auth_service,
            character_service,
            jwt_service,
            item_service,
            inventory_service,
        }
    }
}

pub async fn run(config: Config) {
    let db = Database::new(&config.database.url);
    let state = Arc::new(AppState::new(db, &config));

    let http = {
        let state = state.clone();
        let port = config.server.http_port;

        tokio::spawn(async move {
            // Configure CORS using allowed origins from configuration.
            // We build a CorsLayer that either permits all origins (if "*" present)
            // or uses a dynamic origin checker against the configured list.
            let allowed_origins = config.server.allowed_origins.clone();
            let cors = if allowed_origins.iter().any(|o| o == "*") {
                // permissive for convenience in dev if "*" is used.
                CorsLayer::permissive()
            } else {
                let allowed = allowed_origins.clone();
                let origin_vals: Vec<HeaderValue> = allowed
                    .iter()
                    .filter_map(|a| HeaderValue::from_str(a).ok())
                    .collect();

                CorsLayer::new()
                    .allow_origin(origin_vals)
                    .allow_methods([
                        Method::GET,
                        Method::POST,
                        Method::PUT,
                        Method::DELETE,
                        Method::OPTIONS,
                        Method::PATCH,
                    ])
                    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
                    .allow_credentials(true)
            };

            let app = Router::new()
                .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
                .merge(routes::auth::router())
                .merge(routes::admin::router())
                .with_state(state)
                .layer(cors);

            let addr = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(&addr)
                .await
                .expect("Failed to bind address");

            info!("Server listening on {addr}");
            info!("API docs available at http://{addr}/docs");

            axum::serve(listener, app).await.expect("Server error");
        })
    };

    let grpc = {
        let state = state.clone();
        let port = config.server.grpc_port;

        tokio::spawn(async move {
            let addr = format!("0.0.0.0:{}", port);

            info!("gRPC server listening on {addr}");

            let grpc_auth_service = GrpcAuthServiceImpl::new(state.auth_service.clone());
            let grpc_character_service = GrpcCharacterServiceImpl::new(
                state.auth_service.clone(),
                state.character_service.clone(),
            );
            let grpc_inventory_service = GrpcInventoryServiceImpl::new(
                state.auth_service.clone(),
                state.inventory_service.clone(),
                state.character_service.clone(),
            );

            Server::builder()
                .add_service(
                    ReflectionBuilder::configure()
                        .register_encoded_file_descriptor_set(shared::proto::FILE_DESCRIPTOR_SET)
                        .build_v1()
                        .unwrap(),
                )
                .add_service(AuthServiceServer::new(grpc_auth_service))
                .add_service(CharacterServiceServer::new(grpc_character_service))
                .add_service(InventoryServiceServer::new(grpc_inventory_service))
                .serve(addr.parse().unwrap())
                .await
                .expect("gRPC server error");
        })
    };

    let _ = tokio::join!(http, grpc);
}
