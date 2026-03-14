use std::sync::Arc;

use axum::Router;
use http::header::HeaderValue;
use http::{Method, header};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::app::openapi::ApiDoc;
use crate::app::state::AppState;
use crate::config::Config;
use crate::routes;

pub async fn serve_http(state: Arc<AppState>, config: Config, addr: &str) {
    let allowed_origins = config.server.allowed_origins.clone();
    let cors = if allowed_origins.iter().any(|o| o == "*") {
        CorsLayer::permissive()
    } else {
        let origin_vals: Vec<HeaderValue> = allowed_origins
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

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    info!("Server listening on {addr}");
    info!("API docs available at http://{addr}/docs");

    axum::serve(listener, app).await.expect("Server error");
}
