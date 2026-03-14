use bevy::prelude::Resource;
use shared::net::protocol_id_from_version;

#[derive(Debug, Clone, Resource)]
pub struct ServerRuntimeConfig {
    pub server_addr: &'static str,
    pub api_server_grpc_addr: String,
    pub protocol_id: u64,
    pub max_clients: usize,
    pub tick_rate: f64,
    pub internal_server_token: String,
    pub jwt_secret_game: String,
}

impl ServerRuntimeConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            server_addr: "0.0.0.0:5000",
            api_server_grpc_addr: "http://127.0.0.1:50051".to_string(),
            protocol_id: protocol_id_from_version(env!("CARGO_PKG_VERSION")),
            max_clients: 64,
            tick_rate: 60.0,
            internal_server_token: std::env::var("API_INTERNAL_SERVER_TOKEN")
                .expect("missing API_INTERNAL_SERVER_TOKEN for game-server"),
            jwt_secret_game: std::env::var("API_JWT_SECRET_GAME")
                .expect("missing API_JWT_SECRET_GAME for game-server"),
        }
    }
}
