use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use bevy_renet::netcode::NetcodeServerPlugin;

use crate::events::player_connection::{PlayerConnected, PlayerDisconnected};

use super::systems;

pub struct GameServerNetworkPlugin;

impl Plugin for GameServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerConnected>()
            .add_message::<PlayerDisconnected>()
            .add_plugins(RenetServerPlugin)
            .add_plugins(NetcodeServerPlugin)
            .add_systems(Startup, systems::setup_server)
            .add_systems(
                Update,
                (
                    systems::log_server_started,
                    systems::initialize_session_on_player_connected,
                    systems::cleanup_session_on_player_disconnected,
                    systems::process_client_messages,
                    systems::tick_runtime_characters,
                    systems::flush_player_connected_messages,
                    systems::flush_player_disconnected_messages,
                ),
            )
            .add_observer(systems::handle_renet_server_events);
    }
}
