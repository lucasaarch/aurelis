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
use crate::resources::internal_api::{
    CharacterSummaryView as InternalCharacterSummaryView, InternalApi,
};
use crate::resources::runtime_characters::RuntimeCharacters;
use crate::resources::server_boot_state::ServerBootState;
use crate::runtime::builder::build_runtime_character;
use crate::runtime::client_snapshot::{
    build_character_snapshot_view, build_character_stats_snapshot, build_equipped_views,
    build_inventory_views,
};
use crate::runtime::equipment_change::{equip_item, unequip_item};
use crate::runtime::gem_socket::socket_gem;
use crate::runtime::refinement::{RefinementOutcome, refine_equipment};
use crate::runtime::use_item::{UseItemRequest, use_item};
use crate::runtime::use_skill::use_skill;
use crate::server_config::ServerRuntimeConfig;
use shared::net::{CharacterSummaryView, ClientMessage, ServerMessage};

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
    mut runtime_characters: ResMut<RuntimeCharacters>,
) {
    for event in reader.read() {
        sessions.by_client_id.remove(&event.client_id);
        runtime_characters.by_client_id.remove(&event.client_id);
    }
}

pub fn process_client_messages(
    mut server: ResMut<RenetServer>,
    mut sessions: ResMut<ClientSessions>,
    mut runtime_characters: ResMut<RuntimeCharacters>,
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
                ClientMessage::Login { email, password } => {
                    handle_login(
                        &mut server,
                        &mut sessions,
                        &internal_api,
                        client_id,
                        email,
                        password,
                    );
                }
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
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        character_id,
                    );
                }
                ClientMessage::ListCharacters => {
                    handle_list_characters(&mut server, &sessions, &internal_api, client_id);
                }
                ClientMessage::CreateCharacter { name, class_slug } => {
                    handle_create_character(
                        &mut server,
                        &sessions,
                        &internal_api,
                        client_id,
                        name,
                        class_slug,
                    );
                }
                ClientMessage::UseItem {
                    inventory_type,
                    slot,
                } => {
                    handle_use_item(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        inventory_type,
                        slot,
                    );
                }
                ClientMessage::UseItemOnEquipment {
                    inventory_type,
                    slot,
                    equipment_slot,
                } => {
                    handle_use_item_on_equipment(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        inventory_type,
                        slot,
                        equipment_slot,
                    );
                }
                ClientMessage::UseSkill { skill_slug } => {
                    handle_use_skill(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        client_id,
                        skill_slug,
                    );
                }
                ClientMessage::EquipItem {
                    inventory_type,
                    slot,
                } => {
                    handle_equip_item(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        inventory_type,
                        slot,
                    );
                }
                ClientMessage::UnequipItem { equipment_slot } => {
                    handle_unequip_item(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        equipment_slot,
                    );
                }
                ClientMessage::RefineEquipment { equipment_slot } => {
                    handle_refine_equipment(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        equipment_slot,
                    );
                }
                ClientMessage::SocketGem {
                    equipment_slot,
                    inventory_type,
                    slot,
                    socket_index,
                } => {
                    handle_socket_gem(
                        &mut server,
                        &sessions,
                        &mut runtime_characters,
                        &internal_api,
                        client_id,
                        equipment_slot,
                        inventory_type,
                        slot,
                        socket_index,
                    );
                }
            }
        }
    }
}

fn handle_login(
    server: &mut RenetServer,
    sessions: &mut ClientSessions,
    internal_api: &InternalApi,
    client_id: u64,
    email: String,
    password: String,
) {
    let Some(session) = sessions.by_client_id.get_mut(&client_id) else {
        return;
    };

    match internal_api.game_login(email, password) {
        Ok(account_id) => {
            session.state = SessionState::Authenticated { account_id };
            send_server_message(
                server,
                client_id,
                &ServerMessage::LoginSucceeded { account_id },
            );
            send_server_message(
                server,
                client_id,
                &ServerMessage::Authenticated { account_id },
            );
            info!("client {} logged in as account {}", client_id, account_id);
        }
        Err(reason) => {
            send_server_message(server, client_id, &ServerMessage::LoginFailed { reason });
        }
    }
}

pub fn tick_runtime_characters(
    mut server: ResMut<RenetServer>,
    mut runtime_characters: ResMut<RuntimeCharacters>,
    config: Res<ServerRuntimeConfig>,
) {
    let elapsed_ms = ((1.0 / config.tick_rate) * 1000.0).round() as u64;
    for (client_id, runtime_character) in runtime_characters.by_client_id.iter_mut() {
        let buffs_changed = runtime_character.tick_timed_modifiers(elapsed_ms);
        let cooldowns_changed = runtime_character.tick_skill_cooldowns(elapsed_ms);
        if buffs_changed || cooldowns_changed {
            send_runtime_state(server.as_mut(), *client_id, runtime_character);
            if buffs_changed {
                send_character_stats(server.as_mut(), *client_id, runtime_character);
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
    runtime_characters: &mut RuntimeCharacters,
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

    let runtime_character = match build_runtime_character(&snapshot) {
        Ok(runtime_character) => runtime_character,
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
    runtime_characters
        .by_client_id
        .insert(client_id, runtime_character.clone());
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
    info!(
        "runtime stats for client {} character {} => base={:?} class={:?} equip={:?} final={:?}",
        client_id,
        runtime_character.character_id,
        runtime_character.stats.base,
        runtime_character.stats.from_class,
        runtime_character.stats.from_equipment,
        runtime_character.stats.final_stats
    );
    send_character_snapshot(server, client_id, &runtime_character, &snapshot);
    send_runtime_state(server, client_id, &runtime_character);
}

fn handle_list_characters(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    internal_api: &InternalApi,
    client_id: u64,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };
    let account_id = match session.state {
        SessionState::Authenticated { account_id }
        | SessionState::CharacterSelected { account_id, .. } => account_id,
        SessionState::ConnectedUnauthenticated => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterListFailed {
                    reason: "client is not authenticated".to_string(),
                },
            );
            return;
        }
    };

    match internal_api.list_characters(account_id) {
        Ok(characters) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharactersListed {
                    characters: characters
                        .into_iter()
                        .map(map_character_summary_view)
                        .collect(),
                },
            );
        }
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterListFailed { reason },
            );
        }
    }
}

fn handle_create_character(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    internal_api: &InternalApi,
    client_id: u64,
    name: String,
    class_slug: String,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };
    let account_id = match session.state {
        SessionState::Authenticated { account_id }
        | SessionState::CharacterSelected { account_id, .. } => account_id,
        SessionState::ConnectedUnauthenticated => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterCreationFailed {
                    reason: "client is not authenticated".to_string(),
                },
            );
            return;
        }
    };

    match internal_api.create_character(account_id, name, class_slug) {
        Ok(character) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterCreated {
                    character: map_character_summary_view(character),
                },
            );
        }
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::CharacterCreationFailed { reason },
            );
        }
    }
}

fn map_character_summary_view(summary: InternalCharacterSummaryView) -> CharacterSummaryView {
    CharacterSummaryView {
        character_id: summary.character_id,
        name: summary.name,
        level: summary.level,
        class_slug: summary.class_slug,
        created_at: summary.created_at,
        updated_at: summary.updated_at,
    }
}

fn handle_use_item(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    internal_api: &InternalApi,
    client_id: u64,
    inventory_type: String,
    slot: i16,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };

    let (account_id, character_id) = match session.state {
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, character_id),
        SessionState::Authenticated { .. } | SessionState::ConnectedUnauthenticated => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::ItemUseFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    };

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(server, client_id, &ServerMessage::ItemUseFailed { reason });
            return;
        }
    };

    let result = match use_item(UseItemRequest {
        api: internal_api,
        snapshot: &snapshot,
        inventory_type: &inventory_type,
        slot,
        target_equipment_slot: None,
    }) {
        Ok(result) => result,
        Err(reason) => {
            send_server_message(server, client_id, &ServerMessage::ItemUseFailed { reason });
            return;
        }
    };

    runtime_characters
        .by_client_id
        .insert(client_id, result.runtime_character);
    send_server_message(
        server,
        client_id,
        &ServerMessage::ItemUsed {
            inventory_type,
            slot,
        },
    );
    if let Some(runtime_character) = runtime_characters.by_client_id.get(&client_id) {
        send_character_inventory(server, client_id, &result.snapshot, runtime_character);
        send_character_stats(server, client_id, runtime_character);
        send_runtime_state(server, client_id, runtime_character);
    }
}

fn handle_use_item_on_equipment(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    internal_api: &InternalApi,
    client_id: u64,
    inventory_type: String,
    slot: i16,
    equipment_slot: String,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };

    let (account_id, character_id) = match session.state {
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, character_id),
        SessionState::Authenticated { .. } | SessionState::ConnectedUnauthenticated => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::ItemUseFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    };

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(server, client_id, &ServerMessage::ItemUseFailed { reason });
            return;
        }
    };

    let result = match use_item(UseItemRequest {
        api: internal_api,
        snapshot: &snapshot,
        inventory_type: &inventory_type,
        slot,
        target_equipment_slot: Some(&equipment_slot),
    }) {
        Ok(result) => result,
        Err(reason) => {
            send_server_message(server, client_id, &ServerMessage::ItemUseFailed { reason });
            return;
        }
    };

    runtime_characters
        .by_client_id
        .insert(client_id, result.runtime_character);
    send_server_message(
        server,
        client_id,
        &ServerMessage::ItemUsedOnEquipment {
            inventory_type,
            slot,
            equipment_slot,
        },
    );
    if let Some(runtime_character) = runtime_characters.by_client_id.get(&client_id) {
        send_character_inventory(server, client_id, &result.snapshot, runtime_character);
        send_character_stats(server, client_id, runtime_character);
        send_runtime_state(server, client_id, runtime_character);
    }
}

fn handle_use_skill(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    client_id: u64,
    skill_slug: String,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };

    match session.state {
        SessionState::CharacterSelected { .. } => {}
        SessionState::Authenticated { .. } | SessionState::ConnectedUnauthenticated => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::SkillUseFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    }

    let Some(runtime_character) = runtime_characters.by_client_id.get_mut(&client_id) else {
        send_server_message(
            server,
            client_id,
            &ServerMessage::SkillUseFailed {
                reason: "runtime character is not loaded".to_string(),
            },
        );
        return;
    };

    if let Err(reason) = use_skill(runtime_character, &skill_slug) {
        send_server_message(server, client_id, &ServerMessage::SkillUseFailed { reason });
        return;
    }

    send_server_message(server, client_id, &ServerMessage::SkillUsed { skill_slug });
    send_character_stats(server, client_id, runtime_character);
    send_runtime_state(server, client_id, runtime_character);
}

fn handle_equip_item(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    internal_api: &InternalApi,
    client_id: u64,
    inventory_type: String,
    slot: i16,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };
    let (account_id, character_id) = match session.state {
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, character_id),
        _ => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    };

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed { reason },
            );
            return;
        }
    };
    let reloaded = match equip_item(internal_api, &snapshot, &inventory_type, slot) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed { reason },
            );
            return;
        }
    };
    let runtime_character = match build_runtime_character(&reloaded) {
        Ok(runtime) => runtime,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed { reason },
            );
            return;
        }
    };
    let equipped_slot = reloaded
        .equipment
        .iter()
        .find_map(|equipment| {
            snapshot
                .inventories
                .iter()
                .find(|inventory| inventory.inventory_type == inventory_type)
                .and_then(|inventory| inventory.items.iter().find(|item| item.slot_index == slot))
                .and_then(|item| item.item_instance_id)
                .filter(|id| *id == equipment.item_instance_id)
                .map(|_| equipment.slot.clone())
        })
        .unwrap_or_else(|| "unknown".to_string());

    runtime_characters
        .by_client_id
        .insert(client_id, runtime_character.clone());
    send_server_message(
        server,
        client_id,
        &ServerMessage::ItemEquipped {
            inventory_type,
            slot,
            equipment_slot: equipped_slot,
        },
    );
    send_character_inventory(server, client_id, &reloaded, &runtime_character);
    send_character_stats(server, client_id, &runtime_character);
    send_runtime_state(server, client_id, &runtime_character);
}

fn handle_unequip_item(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    internal_api: &InternalApi,
    client_id: u64,
    equipment_slot: String,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };
    let (account_id, character_id) = match session.state {
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, character_id),
        _ => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    };

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed { reason },
            );
            return;
        }
    };
    let reloaded = match unequip_item(internal_api, &snapshot, &equipment_slot) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed { reason },
            );
            return;
        }
    };
    let runtime_character = match build_runtime_character(&reloaded) {
        Ok(runtime) => runtime,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentChangeFailed { reason },
            );
            return;
        }
    };

    runtime_characters
        .by_client_id
        .insert(client_id, runtime_character.clone());
    send_server_message(
        server,
        client_id,
        &ServerMessage::ItemUnequipped { equipment_slot },
    );
    send_character_inventory(server, client_id, &reloaded, &runtime_character);
    send_character_stats(server, client_id, &runtime_character);
    send_runtime_state(server, client_id, &runtime_character);
}

fn handle_refine_equipment(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    internal_api: &InternalApi,
    client_id: u64,
    equipment_slot: String,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };
    let (account_id, character_id) = match session.state {
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, character_id),
        _ => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentRefineFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    };

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentRefineFailed { reason },
            );
            return;
        }
    };

    let (reloaded, outcome) = match refine_equipment(internal_api, &snapshot, &equipment_slot) {
        Ok(value) => value,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentRefineFailed { reason },
            );
            return;
        }
    };
    let runtime_character = match build_runtime_character(&reloaded) {
        Ok(runtime) => runtime,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::EquipmentRefineFailed { reason },
            );
            return;
        }
    };

    runtime_characters
        .by_client_id
        .insert(client_id, runtime_character.clone());

    let (old_refinement, new_refinement, outcome_label) = match outcome {
        RefinementOutcome::Success {
            old_refinement,
            new_refinement,
        } => (old_refinement, new_refinement, "success".to_string()),
        RefinementOutcome::FailedNoChange { old_refinement } => (
            old_refinement,
            old_refinement,
            "failed_no_change".to_string(),
        ),
        RefinementOutcome::FailedReset {
            old_refinement,
            new_refinement,
        } => (old_refinement, new_refinement, "failed_reset".to_string()),
    };

    send_server_message(
        server,
        client_id,
        &ServerMessage::EquipmentRefined {
            equipment_slot,
            old_refinement,
            new_refinement,
            outcome: outcome_label,
        },
    );
    send_character_inventory(server, client_id, &reloaded, &runtime_character);
    send_character_stats(server, client_id, &runtime_character);
    send_runtime_state(server, client_id, &runtime_character);
}

fn handle_socket_gem(
    server: &mut RenetServer,
    sessions: &ClientSessions,
    runtime_characters: &mut RuntimeCharacters,
    internal_api: &InternalApi,
    client_id: u64,
    equipment_slot: String,
    inventory_type: String,
    slot: i16,
    socket_index: i16,
) {
    let Some(session) = sessions.by_client_id.get(&client_id) else {
        return;
    };
    let (account_id, character_id) = match session.state {
        SessionState::CharacterSelected {
            account_id,
            character_id,
        } => (account_id, character_id),
        _ => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::GemSocketFailed {
                    reason: "client has no selected character".to_string(),
                },
            );
            return;
        }
    };

    let snapshot = match internal_api.load_playable_character(account_id, character_id) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::GemSocketFailed { reason },
            );
            return;
        }
    };

    let reloaded = match socket_gem(
        internal_api,
        &snapshot,
        &equipment_slot,
        &inventory_type,
        slot,
        socket_index,
    ) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::GemSocketFailed { reason },
            );
            return;
        }
    };
    let runtime_character = match build_runtime_character(&reloaded) {
        Ok(runtime) => runtime,
        Err(reason) => {
            send_server_message(
                server,
                client_id,
                &ServerMessage::GemSocketFailed { reason },
            );
            return;
        }
    };

    runtime_characters
        .by_client_id
        .insert(client_id, runtime_character.clone());
    send_server_message(
        server,
        client_id,
        &ServerMessage::GemSocketed {
            equipment_slot,
            socket_index,
        },
    );
    send_character_inventory(server, client_id, &reloaded, &runtime_character);
    send_character_stats(server, client_id, &runtime_character);
    send_runtime_state(server, client_id, &runtime_character);
}

fn send_server_message(server: &mut RenetServer, client_id: u64, message: &ServerMessage) {
    let payload = bincode::serialize(message).expect("failed to encode server message");
    server.send_message(client_id, DefaultChannel::ReliableOrdered, payload);
}

fn send_runtime_state(
    server: &mut RenetServer,
    client_id: u64,
    runtime_character: &crate::runtime::character::RuntimeCharacter,
) {
    send_server_message(
        server,
        client_id,
        &ServerMessage::RuntimeStateUpdated {
            current_hp: runtime_character.resources.current_hp,
            current_mp: runtime_character.resources.current_mp,
            active_buffs: runtime_character.active_buffs(),
            skill_cooldowns: runtime_character.skill_cooldowns(),
        },
    );
}

fn send_character_snapshot(
    server: &mut RenetServer,
    client_id: u64,
    runtime_character: &crate::runtime::character::RuntimeCharacter,
    snapshot: &crate::resources::internal_api::PlayableCharacterSnapshot,
) {
    match build_character_snapshot_view(runtime_character, snapshot) {
        Ok(snapshot) => send_server_message(
            server,
            client_id,
            &ServerMessage::CharacterSnapshotLoaded { snapshot },
        ),
        Err(reason) => warn!(
            "failed to build character snapshot for client {} character {}: {}",
            client_id, runtime_character.character_id, reason
        ),
    }
}

fn send_character_stats(
    server: &mut RenetServer,
    client_id: u64,
    runtime_character: &crate::runtime::character::RuntimeCharacter,
) {
    send_server_message(
        server,
        client_id,
        &ServerMessage::CharacterStatsUpdated {
            stats: build_character_stats_snapshot(runtime_character),
        },
    );
}

fn send_character_inventory(
    server: &mut RenetServer,
    client_id: u64,
    snapshot: &crate::resources::internal_api::PlayableCharacterSnapshot,
    runtime_character: &crate::runtime::character::RuntimeCharacter,
) {
    match (
        build_inventory_views(snapshot),
        build_equipped_views(snapshot, runtime_character),
    ) {
        (Ok(inventories), Ok(equipped)) => send_server_message(
            server,
            client_id,
            &ServerMessage::CharacterInventoryUpdated {
                inventories,
                equipped,
            },
        ),
        (Err(reason), _) | (_, Err(reason)) => warn!(
            "failed to build character inventory payload for client {} character {}: {}",
            client_id, runtime_character.character_id, reason
        ),
    }
}
