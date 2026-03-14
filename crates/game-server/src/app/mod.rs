use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;
use tracing::level_filters::LevelFilter;

use crate::network::auth::GameTokenVerifier;
use crate::network::plugin::GameServerNetworkPlugin;
use crate::resources::client_sessions::ClientSessions;
use crate::resources::connected_players::ConnectedPlayers;
use crate::resources::internal_api::InternalApi;
use crate::resources::server_boot_state::ServerBootState;
use crate::server_config::ServerRuntimeConfig;

pub fn run() {
    init_tracing();

    let runtime_config = ServerRuntimeConfig::from_env();
    let tick_rate = runtime_config.tick_rate;
    let token_verifier = GameTokenVerifier::from_secret(&runtime_config.jwt_secret_game);
    let internal_api = InternalApi::new(
        runtime_config.api_server_grpc_addr.clone(),
        runtime_config.internal_server_token.clone(),
    );

    App::new()
        .insert_resource(runtime_config)
        .insert_resource(internal_api)
        .insert_resource(token_verifier)
        .insert_resource(ServerBootState::default())
        .insert_resource(ClientSessions::default())
        .insert_resource(ConnectedPlayers::default())
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / tick_rate,
            ))),
        )
        .add_plugins(GameServerNetworkPlugin)
        .run();
}

fn init_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .json()
        .try_init();
}
