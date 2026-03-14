use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::RenetServer;
use bevy_renet::RenetServerEvent;
use bevy_renet::netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, ServerEvent};
use tracing::{info, warn};

use crate::events::player_connection::{PlayerConnected, PlayerDisconnected};
use crate::network::auth::GameTokenVerifier;
use crate::resources::client_sessions::{ClientSession, ClientSessions, SessionState};
use crate::resources::connected_players::ConnectedPlayers;
use crate::resources::internal_api::InternalApi;
use crate::resources::server_boot_state::ServerBootState;
use crate::server_config::ServerRuntimeConfig;
use shared::net::{ClientMessage, ServerMessage};

pub fn setup_server(mut commands: Commands, config: Res<ServerRuntimeConfig>) {
    let server_addr: SocketAddr = config
        .server_addr
        .parse()
        .expect("invalid game server address");
    let socket = UdpSocket::bind(server_addr).expect("failed to bind UDP socket");
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system clock before unix epoch");

    let server = RenetServer::new(ConnectionConfig::default());
    let transport = NetcodeServerTransport::new(
        ServerConfig {
            current_time,
            max_clients: config.max_clients,
            protocol_id: config.protocol_id,
            public_addresses: vec![server_addr],
            authentication: ServerAuthentication::Unsecure,
        },
        socket,
    )
    .expect("failed to create netcode transport");

    commands.insert_resource(server);
    commands.insert_resource(transport);
}

pub fn log_server_started(
    mut boot_state: ResMut<ServerBootState>,
    config: Res<ServerRuntimeConfig>,
) {
    if boot_state.announced {
        return;
    }

    info!(
        "game-server listening on udp://{} with Bevy 0.18 / bevy_renet 4.0",
        config.server_addr
    );
    boot_state.announced = true;
}

pub fn handle_renet_server_events(
    server_event: On<RenetServerEvent>,
    mut connected_players: ResMut<ConnectedPlayers>,
    mut connected_writer: MessageWriter<PlayerConnected>,
    mut disconnected_writer: MessageWriter<PlayerDisconnected>,
) {
    match **server_event {
        ServerEvent::ClientConnected { client_id } => {
            connected_players.client_ids.insert(client_id);
            connected_writer.write(PlayerConnected { client_id });
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
            connected_players.client_ids.remove(&client_id);
            disconnected_writer.write(PlayerDisconnected {
                client_id,
                reason: reason.to_string(),
            });
        }
    }
}

pub fn flush_player_connected_messages(mut reader: MessageReader<PlayerConnected>) {
    for event in reader.read() {
        info!("client connected: {}", event.client_id);
    }
}

pub fn flush_player_disconnected_messages(mut reader: MessageReader<PlayerDisconnected>) {
    for event in reader.read() {
        info!(
            "client disconnected: {} ({})",
            event.client_id, event.reason
        );
    }
}

pub fn initialize_session_on_player_connected(
    mut reader: MessageReader<PlayerConnected>,
    mut sessions: ResMut<ClientSessions>,
) {
    for event in reader.read() {
        sessions.by_client_id.insert(
            event.client_id,
            ClientSession {
                state: SessionState::ConnectedUnauthenticated,
            },
        );
    }
}

pub fn cleanup_session_on_player_disconnected(
    mut reader: MessageReader<PlayerDisconnected>,
    mut sessions: ResMut<ClientSessions>,
) {
    for event in reader.read() {
        sessions.by_client_id.remove(&event.client_id);
    }
}

pub fn process_client_messages(
    mut server: ResMut<RenetServer>,
    mut sessions: ResMut<ClientSessions>,
    internal_api: Res<InternalApi>,
    token_verifier: Res<GameTokenVerifier>,
) {
    let client_ids = server.clients_id().into_iter().collect::<Vec<_>>();

    for client_id in client_ids {
        while let Some(payload) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let Ok(message) = bincode::deserialize::<ClientMessage>(&payload) else {
                warn!("failed to decode client message from client {}", client_id);
                continue;
            };

            match message {
                ClientMessage::Authenticate { token } => {
                    handle_authenticate(
                        &mut server,
                        &mut sessions,
                        &token_verifier,
                        client_id,
                        token,
                    );
                }
                ClientMessage::SelectCharacter { character_id } => {
                    handle_select_character(
                        &mut server,
                        &mut sessions,
                        &internal_api,
                        client_id,
                        character_id,
                    );
                }
            }
        }
    }
}

fn handle_authenticate(
    server: &mut RenetServer,
    sessions: &mut ClientSessions,
    token_verifier: &GameTokenVerifier,
    client_id: u64,
    token: String,
) {
    let Some(session) = sessions.by_client_id.get_mut(&client_id) else {
        return;
    };

    match token_verifier.verify_account_id(&token) {
        Ok(account_id) => {
            session.state = SessionState::Authenticated { account_id };
            send_server_message(
                server,
                client_id,
                &ServerMessage::Authenticated { account_id },
            );
            info!(
                "client {} authenticated as account {}",
                client_id, account_id
            );
        }
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::AuthenticationFailed {
                    reason: "invalid game token".to_string(),
                },
            );
            warn!("client {} failed authentication: {}", client_id, reason);
        }
    }
}

fn handle_select_character(
    server: &mut RenetServer,
    sessions: &mut ClientSessions,
    internal_api: &InternalApi,
    client_id: u64,
    character_id: uuid::Uuid,
) {
    let Some(session) = sessions.by_client_id.get_mut(&client_id) else {
        return;
    };

    let (account_id, previous_character_id) = match session.state.clone() {
        SessionState::Authenticated { account_id } => (account_id, None),
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, Some(character_id)),
        SessionState::ConnectedUnauthenticated => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterSelectionFailed {
                    reason: "client is not authenticated".to_string(),
                },
            );
            return;
        }
    };

    if character_id.is_nil() {
        send_server_message(
            server,
            client_id,
            &ServerMessage::CharacterSelectionFailed {
                reason: "character id cannot be nil".to_string(),
            },
        );
        return;
    }

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterSelectionFailed { reason },
            );
            return;
        }
    };

    session.state = SessionState::CharacterSelected {
        account_id,
        character_id: snapshot.character_id,
    };
    send_server_message(
        server,
        client_id,
        &ServerMessage::CharacterSelected {
            character_id: snapshot.character_id,
        },
    );
    if let Some(previous_character_id) = previous_character_id {
        info!(
            "client {} switched character from {} to {} for account {}",
            client_id, previous_character_id, snapshot.character_id, account_id
        );
    } else {
        info!(
            "client {} selected character {} ('{}', base='{}', class='{}', level={}) for account {}",
            client_id,
            snapshot.character_id,
            snapshot.name,
            snapshot.base_character_slug,
            snapshot.current_class_slug,
            snapshot.level,
            account_id
        );
    }
}

fn send_server_message(server: &mut RenetServer, client_id: u64, message: &ServerMessage) {
    let payload = bincode::serialize(message).expect("failed to encode server message");
    server.send_message(client_id, DefaultChannel::ReliableOrdered, payload);
}
