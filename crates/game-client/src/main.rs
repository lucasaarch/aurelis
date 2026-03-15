use std::env;
use std::fmt::Write as _;
use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::audio::AudioPlugin;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{Backends, WgpuSettings},
};
use bevy::window::WindowResolution;
use bevy_renet::client_connected;
use bevy_renet::netcode::{
    ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeErrorEvent,
};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel};
use bevy_renet::{RenetClient, RenetClientPlugin};
use shared::net::{
    CharacterSnapshotView, CharacterStatsSnapshot, CharacterSummaryView, ClientMessage,
    EquippedSlotView, InventoryView, ServerMessage, protocol_id_from_version,
};
use uuid::Uuid;

const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:5000";

fn main() {
    let config = ClientConfig::from_env();
    let (client, transport) = new_client(&config);
    let mut plugins = DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
            title: "Aurelis Character Viewer".to_string(),
            resolution: WindowResolution::new(1440, 920),
            ..default()
        }),
        ..default()
    });
    if let Some(backends) = config.preferred_backends {
        plugins = plugins.set(RenderPlugin {
            render_creation: WgpuSettings {
                backends: Some(backends),
                ..default()
            }
            .into(),
            ..default()
        });
    }
    if !config.enable_audio {
        plugins = plugins.disable::<AudioPlugin>();
    }

    App::new()
        .add_plugins(plugins)
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .insert_resource(client)
        .insert_resource(transport)
        .insert_resource(config.clone())
        .insert_resource(ClientBootState::default())
        .insert_resource(ClientRuntimeState::default())
        .insert_resource(AppScreen::default())
        .insert_resource(LoginForm::from_config(&config))
        .insert_resource(CharacterCreateForm::default())
        .insert_resource(CharacterListState::default())
        .insert_resource(StatusMessage::default())
        .insert_resource(UiDirty(true))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, log_client_started)
        .add_systems(Update, log_connected.run_if(client_connected))
        .add_systems(Update, process_server_messages)
        .add_systems(Update, send_startup_messages.run_if(client_connected))
        .add_systems(Update, maybe_send_auto_action.run_if(client_connected))
        .add_systems(Update, handle_ui_buttons)
        .add_systems(Update, handle_text_input)
        .add_systems(Update, rebuild_ui)
        .add_systems(Update, log_disconnected)
        .add_observer(handle_netcode_error)
        .run();
}

#[derive(Resource, Clone)]
struct ClientConfig {
    server_addr: String,
    email: Option<String>,
    password: Option<String>,
    token: Option<String>,
    character_id: Option<Uuid>,
    auto_action: Option<AutoAction>,
    enable_audio: bool,
    preferred_backends: Option<Backends>,
}

impl ClientConfig {
    fn from_env() -> Self {
        let enable_audio = env::var("GAME_CLIENT_ENABLE_AUDIO")
            .ok()
            .as_deref()
            .map(|value| matches!(value, "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(true);
        let preferred_backends = None;

        Self {
            server_addr: env::var("GAME_SERVER_ADDR")
                .unwrap_or_else(|_| DEFAULT_SERVER_ADDR.to_string()),
            email: env::var("GAME_CLIENT_EMAIL")
                .ok()
                .filter(|value| !value.is_empty()),
            password: env::var("GAME_CLIENT_PASSWORD")
                .ok()
                .filter(|value| !value.is_empty()),
            token: env::var("GAME_CLIENT_TOKEN")
                .ok()
                .filter(|value| !value.is_empty()),
            character_id: env::var("GAME_CLIENT_CHARACTER_ID")
                .ok()
                .and_then(|value| Uuid::parse_str(&value).ok()),
            auto_action: env::var("GAME_CLIENT_AUTO_ACTION")
                .ok()
                .and_then(|value| AutoAction::parse(&value)),
            enable_audio,
            preferred_backends,
        }
    }
}

#[derive(Clone)]
enum AutoAction {
    UseItem {
        inventory_type: String,
        slot: i16,
    },
    UseItemOnEquipment {
        inventory_type: String,
        slot: i16,
        equipment_slot: String,
    },
    EquipItem {
        inventory_type: String,
        slot: i16,
    },
    UnequipItem {
        equipment_slot: String,
    },
    RefineEquipment {
        equipment_slot: String,
    },
    SocketGem {
        equipment_slot: String,
        inventory_type: String,
        slot: i16,
        socket_index: i16,
    },
}

impl AutoAction {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(':').collect::<Vec<_>>();
        match parts.as_slice() {
            ["use_item", inventory_type, slot] => Some(Self::UseItem {
                inventory_type: (*inventory_type).to_string(),
                slot: slot.parse().ok()?,
            }),
            [
                "use_item_on_equipment",
                inventory_type,
                slot,
                equipment_slot,
            ] => Some(Self::UseItemOnEquipment {
                inventory_type: (*inventory_type).to_string(),
                slot: slot.parse().ok()?,
                equipment_slot: (*equipment_slot).to_string(),
            }),
            ["equip_item", inventory_type, slot] => Some(Self::EquipItem {
                inventory_type: (*inventory_type).to_string(),
                slot: slot.parse().ok()?,
            }),
            ["unequip_item", equipment_slot] => Some(Self::UnequipItem {
                equipment_slot: (*equipment_slot).to_string(),
            }),
            ["refine_equipment", equipment_slot] => Some(Self::RefineEquipment {
                equipment_slot: (*equipment_slot).to_string(),
            }),
            [
                "socket_gem",
                equipment_slot,
                inventory_type,
                slot,
                socket_index,
            ] => Some(Self::SocketGem {
                equipment_slot: (*equipment_slot).to_string(),
                inventory_type: (*inventory_type).to_string(),
                slot: slot.parse().ok()?,
                socket_index: socket_index.parse().ok()?,
            }),
            _ => None,
        }
    }

    fn to_message(&self) -> ClientMessage {
        match self {
            AutoAction::UseItem {
                inventory_type,
                slot,
            } => ClientMessage::UseItem {
                inventory_type: inventory_type.clone(),
                slot: *slot,
            },
            AutoAction::UseItemOnEquipment {
                inventory_type,
                slot,
                equipment_slot,
            } => ClientMessage::UseItemOnEquipment {
                inventory_type: inventory_type.clone(),
                slot: *slot,
                equipment_slot: equipment_slot.clone(),
            },
            AutoAction::EquipItem {
                inventory_type,
                slot,
            } => ClientMessage::EquipItem {
                inventory_type: inventory_type.clone(),
                slot: *slot,
            },
            AutoAction::UnequipItem { equipment_slot } => ClientMessage::UnequipItem {
                equipment_slot: equipment_slot.clone(),
            },
            AutoAction::RefineEquipment { equipment_slot } => ClientMessage::RefineEquipment {
                equipment_slot: equipment_slot.clone(),
            },
            AutoAction::SocketGem {
                equipment_slot,
                inventory_type,
                slot,
                socket_index,
            } => ClientMessage::SocketGem {
                equipment_slot: equipment_slot.clone(),
                inventory_type: inventory_type.clone(),
                slot: *slot,
                socket_index: *socket_index,
            },
        }
    }
}

#[derive(Resource, Default)]
struct ClientBootState {
    announced: bool,
    connected_once: bool,
    auth_sent: bool,
    select_sent: bool,
    auto_action_sent: bool,
}

#[derive(Resource, Default)]
struct ClientRuntimeState {
    account_id: Option<Uuid>,
    character_snapshot: Option<CharacterSnapshotView>,
    stats: Option<CharacterStatsSnapshot>,
    inventories: Vec<InventoryView>,
    equipped: Vec<EquippedSlotView>,
}

#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
enum AppScreen {
    #[default]
    Login,
    CharacterSelect,
    Viewer,
}

#[derive(Resource, Default)]
struct StatusMessage(String);

#[derive(Resource, Default)]
struct UiDirty(bool);

#[derive(Resource, Default)]
struct CharacterListState {
    characters: Vec<CharacterSummaryView>,
}

#[derive(Resource, Default)]
struct CharacterCreateForm {
    name: String,
    class_slug: String,
    focused: CreateField,
}

#[derive(Resource, Default)]
struct LoginForm {
    email: String,
    password: String,
    focused: LoginField,
}

impl LoginForm {
    fn from_config(config: &ClientConfig) -> Self {
        Self {
            email: config.email.clone().unwrap_or_default(),
            password: config.password.clone().unwrap_or_default(),
            focused: LoginField::Email,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum LoginField {
    #[default]
    Email,
    Password,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum CreateField {
    #[default]
    Name,
    ClassSlug,
}

#[derive(Component)]
struct ScreenRoot;

#[derive(Component, Clone)]
enum UiAction {
    FocusLogin(LoginField),
    SubmitLogin,
    RefreshCharacters,
    FocusCreate(CreateField),
    SubmitCreateCharacter,
    SelectCharacter(Uuid),
    BackToCharacters,
}

fn new_client(config: &ClientConfig) -> (RenetClient, NetcodeClientTransport) {
    let server_addr = config.server_addr.parse().expect("invalid server address");
    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind client UDP socket");
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system clock before unix epoch");
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: protocol_id_from_version(env!("CARGO_PKG_VERSION")),
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .expect("failed to create netcode client transport");
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn panel() -> impl Bundle {
    (
        Node {
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(12.0)),
            overflow: Overflow::clip_y(),
            ..default()
        },
        BackgroundColor(Color::srgb(0.10, 0.11, 0.13)),
    )
}

fn plain_text(value: impl Into<String>, font_size: f32) -> impl Bundle {
    (
        Text::new(value.into()),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.92, 0.94)),
    )
}

fn button(action: UiAction, label: impl Into<String>) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(220.0),
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.18, 0.25, 0.32)),
        action,
        Name::new(label.into()),
    )
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    action: UiAction,
    label: impl Into<String>,
) {
    let label = label.into();
    parent
        .spawn(button(action, label.clone()))
        .with_children(|parent| {
            parent.spawn(plain_text(label, 16.0));
        });
}

fn spawn_login_screen(parent: &mut ChildSpawnerCommands, login: &LoginForm) {
    parent
        .spawn((
            Node {
                width: Val::Px(520.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(18.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.11, 0.13)),
        ))
        .with_children(|parent| {
            parent.spawn(plain_text("Login", 22.0));
            parent.spawn(plain_text(
                "Tab alternates fields. Type directly into the selected field.",
                14.0,
            ));

            let email_label = if login.focused == LoginField::Email {
                format!("> Email: {}", login.email)
            } else {
                format!("Email: {}", login.email)
            };
            spawn_action_button(parent, UiAction::FocusLogin(LoginField::Email), email_label);

            let masked_password = "*".repeat(login.password.chars().count());
            let password_label = if login.focused == LoginField::Password {
                format!("> Password: {}", masked_password)
            } else {
                format!("Password: {}", masked_password)
            };
            spawn_action_button(
                parent,
                UiAction::FocusLogin(LoginField::Password),
                password_label,
            );

            spawn_action_button(parent, UiAction::SubmitLogin, "Login");
        });
}

fn spawn_character_select_screen(
    parent: &mut ChildSpawnerCommands,
    create_form: &CharacterCreateForm,
    characters: &CharacterListState,
) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            column_gap: Val::Px(16.0),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(55.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(16.0)),
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.11, 0.13)),
                ))
                .with_children(|parent| {
                    parent.spawn(plain_text("Characters", 22.0));
                    spawn_action_button(parent, UiAction::RefreshCharacters, "Refresh list");

                    if characters.characters.is_empty() {
                        parent.spawn(plain_text("No characters loaded.", 16.0));
                    } else {
                        for character in &characters.characters {
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::bottom(Val::Px(8.0)),
                                        row_gap: Val::Px(6.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.14, 0.15, 0.18)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(plain_text(
                                        format!(
                                            "{} | lvl {} | {}",
                                            character.name, character.level, character.class_slug
                                        ),
                                        16.0,
                                    ));
                                    parent.spawn(plain_text(
                                        format!("Character ID: {}", character.character_id),
                                        13.0,
                                    ));
                                    spawn_action_button(
                                        parent,
                                        UiAction::SelectCharacter(character.character_id),
                                        "Select",
                                    );
                                });
                        }
                    }
                });

            parent
                .spawn((
                    Node {
                        width: Val::Percent(45.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(16.0)),
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.11, 0.13)),
                ))
                .with_children(|parent| {
                    parent.spawn(plain_text("Create Character", 22.0));
                    parent.spawn(plain_text(
                        "Use the focused field below, then submit.",
                        14.0,
                    ));

                    let name_label = if create_form.focused == CreateField::Name {
                        format!("> Name: {}", create_form.name)
                    } else {
                        format!("Name: {}", create_form.name)
                    };
                    spawn_action_button(
                        parent,
                        UiAction::FocusCreate(CreateField::Name),
                        name_label,
                    );

                    let class_label = if create_form.focused == CreateField::ClassSlug {
                        format!("> Class Slug: {}", create_form.class_slug)
                    } else {
                        format!("Class Slug: {}", create_form.class_slug)
                    };
                    spawn_action_button(
                        parent,
                        UiAction::FocusCreate(CreateField::ClassSlug),
                        class_label,
                    );

                    parent.spawn(plain_text("Example class slug: kael", 13.0));
                    spawn_action_button(
                        parent,
                        UiAction::SubmitCreateCharacter,
                        "Create character",
                    );
                });
        });
}

fn spawn_viewer_screen(parent: &mut ChildSpawnerCommands, snapshot: &CharacterSnapshotView) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(plain_text(
                format!(
                    "{} | lvl {} | {} | affinity {:?}",
                    snapshot.name, snapshot.level, snapshot.current_class_slug, snapshot.affinity
                ),
                18.0,
            ));
            spawn_action_button(parent, UiAction::BackToCharacters, "Back to characters");
        });

    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(panel()).with_children(|parent| {
                parent.spawn(plain_text(format_stats(snapshot), 15.0));
            });
            parent.spawn(panel()).with_children(|parent| {
                parent.spawn(plain_text(format_equipped(snapshot), 15.0));
            });
            parent.spawn(panel()).with_children(|parent| {
                parent.spawn(plain_text(format_inventories(snapshot), 15.0));
            });
            parent.spawn(panel()).with_children(|parent| {
                parent.spawn(plain_text(format_runtime(snapshot), 15.0));
            });
        });
}

fn log_client_started(config: Res<ClientConfig>, mut boot_state: ResMut<ClientBootState>) {
    if boot_state.announced {
        return;
    }

    println!("game-client connecting to udp://{}", config.server_addr);
    if let Some(backends) = config.preferred_backends {
        println!("render backends forced to {:?}", backends);
    }
    if !config.enable_audio {
        println!("audio disabled for this client session");
    }
    if config.email.is_none() || config.password.is_none() {
        println!("GAME_CLIENT_EMAIL / GAME_CLIENT_PASSWORD are not fully set");
    }
    if config.token.is_none() {
        println!("GAME_CLIENT_TOKEN is not set; token auth fallback is unavailable");
    }
    if config.character_id.is_none() {
        println!("GAME_CLIENT_CHARACTER_ID is not set; client will not select a character");
    }
    boot_state.announced = true;
}

fn log_connected(mut boot_state: ResMut<ClientBootState>) {
    if boot_state.connected_once {
        return;
    }

    boot_state.connected_once = true;
    println!("game-client connected");
}

fn send_startup_messages(
    mut client: ResMut<RenetClient>,
    config: Res<ClientConfig>,
    runtime_state: Res<ClientRuntimeState>,
    mut boot_state: ResMut<ClientBootState>,
) {
    if !boot_state.auth_sent {
        if let (Some(email), Some(password)) = (&config.email, &config.password) {
            send_client_message(
                &mut client,
                &ClientMessage::Login {
                    email: email.clone(),
                    password: password.clone(),
                },
            );
            println!("sent Login");
        } else if let Some(token) = &config.token {
            send_client_message(
                &mut client,
                &ClientMessage::Authenticate {
                    token: token.clone(),
                },
            );
            println!("sent Authenticate");
        } else {
            return;
        }
        boot_state.auth_sent = true;
        return;
    }

    if !boot_state.select_sent && runtime_state.account_id.is_some() {
        let Some(character_id) = config.character_id else {
            return;
        };
        send_client_message(
            &mut client,
            &ClientMessage::SelectCharacter { character_id },
        );
        println!("sent SelectCharacter {}", character_id);
        boot_state.select_sent = true;
    }
}

fn maybe_send_auto_action(
    mut client: ResMut<RenetClient>,
    config: Res<ClientConfig>,
    runtime_state: Res<ClientRuntimeState>,
    mut boot_state: ResMut<ClientBootState>,
) {
    if boot_state.auto_action_sent || runtime_state.character_snapshot.is_none() {
        return;
    }
    let Some(action) = &config.auto_action else {
        return;
    };

    send_client_message(&mut client, &action.to_message());
    println!("sent auto action");
    boot_state.auto_action_sent = true;
}

fn process_server_messages(
    mut client: ResMut<RenetClient>,
    mut screen: ResMut<AppScreen>,
    mut runtime_state: ResMut<ClientRuntimeState>,
    mut characters: ResMut<CharacterListState>,
    mut status: ResMut<StatusMessage>,
    mut dirty: ResMut<UiDirty>,
) {
    while let Some(payload) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let Ok(message) = bincode::deserialize::<ServerMessage>(&payload) else {
            eprintln!("game-client failed to decode server message");
            continue;
        };

        match message {
            ServerMessage::LoginSucceeded { account_id } => {
                println!("login succeeded for account {}", account_id);
                runtime_state.account_id = Some(account_id);
                *screen = AppScreen::CharacterSelect;
                status.0 = "Login succeeded".to_string();
                dirty.0 = true;
                send_client_message(&mut client, &ClientMessage::ListCharacters);
            }
            ServerMessage::LoginFailed { reason } => {
                status.0 = reason.clone();
                dirty.0 = true;
                eprintln!("login failed: {reason}");
            }
            ServerMessage::Authenticated { account_id } => {
                println!("authenticated as account {}", account_id);
                runtime_state.account_id = Some(account_id);
                *screen = AppScreen::CharacterSelect;
                status.0 = "Authenticated with token fallback".to_string();
                dirty.0 = true;
                send_client_message(&mut client, &ClientMessage::ListCharacters);
            }
            ServerMessage::AuthenticationFailed { reason } => {
                status.0 = reason.clone();
                dirty.0 = true;
                eprintln!("authentication failed: {reason}");
            }
            ServerMessage::CharacterSelected { character_id } => {
                println!("character selected {}", character_id);
                status.0 = format!("Selected character {character_id}");
                dirty.0 = true;
            }
            ServerMessage::CharactersListed {
                characters: listed_characters,
            } => {
                println!("characters listed: {}", listed_characters.len());
                for character in &listed_characters {
                    println!(
                        "  {} {} lvl {} {}",
                        character.character_id,
                        character.name,
                        character.level,
                        character.class_slug
                    );
                }
                characters.characters = listed_characters;
                *screen = AppScreen::CharacterSelect;
                status.0 = "Character list loaded".to_string();
                dirty.0 = true;
            }
            ServerMessage::CharacterListFailed { reason } => {
                status.0 = reason.clone();
                dirty.0 = true;
                eprintln!("character list failed: {reason}");
            }
            ServerMessage::CharacterCreated { character } => {
                println!(
                    "character created: {} {} lvl {} {}",
                    character.character_id, character.name, character.level, character.class_slug
                );
                status.0 = format!("Created character {}", character.name);
                dirty.0 = true;
                send_client_message(&mut client, &ClientMessage::ListCharacters);
            }
            ServerMessage::CharacterCreationFailed { reason } => {
                status.0 = reason.clone();
                dirty.0 = true;
                eprintln!("character creation failed: {reason}");
            }
            ServerMessage::CharacterSelectionFailed { reason } => {
                status.0 = reason.clone();
                dirty.0 = true;
                eprintln!("character selection failed: {reason}");
            }
            ServerMessage::CharacterSnapshotLoaded { snapshot } => {
                println!(
                    "snapshot loaded: {} lvl {} class {}",
                    snapshot.name, snapshot.level, snapshot.current_class_slug
                );
                runtime_state.stats = Some(snapshot.stats.clone());
                runtime_state.inventories = snapshot.inventories.clone();
                runtime_state.equipped = snapshot.equipped.clone();
                runtime_state.character_snapshot = Some(snapshot);
                *screen = AppScreen::Viewer;
                status.0.clear();
                dirty.0 = true;
            }
            ServerMessage::CharacterStatsUpdated { stats } => {
                runtime_state.stats = Some(stats);
                dirty.0 = true;
            }
            ServerMessage::CharacterInventoryUpdated {
                inventories,
                equipped,
            } => {
                runtime_state.inventories = inventories;
                runtime_state.equipped = equipped;
                dirty.0 = true;
            }
            ServerMessage::RuntimeStateUpdated {
                current_hp,
                current_mp,
                active_buffs,
                skill_cooldowns,
            } => {
                if let Some(stats) = runtime_state.stats.as_mut() {
                    stats.current_hp = current_hp;
                    stats.current_mp = current_mp;
                    stats.active_buffs = active_buffs;
                    stats.skill_cooldowns = skill_cooldowns;
                }
                dirty.0 = true;
            }
            ServerMessage::ItemUsed { .. }
            | ServerMessage::ItemUsedOnEquipment { .. }
            | ServerMessage::SkillUsed { .. }
            | ServerMessage::ItemEquipped { .. }
            | ServerMessage::ItemUnequipped { .. }
            | ServerMessage::EquipmentRefined { .. }
            | ServerMessage::GemSocketed { .. } => {
                dirty.0 = true;
            }
            ServerMessage::ItemUseFailed { reason }
            | ServerMessage::SkillUseFailed { reason }
            | ServerMessage::EquipmentChangeFailed { reason }
            | ServerMessage::EquipmentRefineFailed { reason }
            | ServerMessage::GemSocketFailed { reason } => {
                status.0 = reason.clone();
                dirty.0 = true;
                eprintln!("{reason}");
            }
        }
    }
}

fn handle_ui_buttons(
    mut client: ResMut<RenetClient>,
    runtime_state: Res<ClientRuntimeState>,
    mut screen: ResMut<AppScreen>,
    mut login: ResMut<LoginForm>,
    mut create_form: ResMut<CharacterCreateForm>,
    mut status: ResMut<StatusMessage>,
    mut dirty: ResMut<UiDirty>,
    query: Query<(&Interaction, &UiAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            UiAction::FocusLogin(field) => {
                login.focused = *field;
                dirty.0 = true;
            }
            UiAction::SubmitLogin => {
                if login.email.trim().is_empty() || login.password.is_empty() {
                    status.0 = "Email and password are required".to_string();
                    dirty.0 = true;
                    continue;
                }
                send_client_message(
                    &mut client,
                    &ClientMessage::Login {
                        email: login.email.clone(),
                        password: login.password.clone(),
                    },
                );
                status.0 = "Logging in...".to_string();
                dirty.0 = true;
            }
            UiAction::RefreshCharacters => {
                send_client_message(&mut client, &ClientMessage::ListCharacters);
                status.0 = "Refreshing character list...".to_string();
                dirty.0 = true;
            }
            UiAction::FocusCreate(field) => {
                create_form.focused = *field;
                dirty.0 = true;
            }
            UiAction::SubmitCreateCharacter => {
                if create_form.name.trim().is_empty() || create_form.class_slug.trim().is_empty() {
                    status.0 = "Character name and class slug are required".to_string();
                    dirty.0 = true;
                    continue;
                }
                send_client_message(
                    &mut client,
                    &ClientMessage::CreateCharacter {
                        name: create_form.name.clone(),
                        class_slug: create_form.class_slug.clone(),
                    },
                );
                status.0 = "Creating character...".to_string();
                dirty.0 = true;
            }
            UiAction::SelectCharacter(character_id) => {
                send_client_message(
                    &mut client,
                    &ClientMessage::SelectCharacter {
                        character_id: *character_id,
                    },
                );
                status.0 = format!("Selecting character {character_id}...");
                dirty.0 = true;
            }
            UiAction::BackToCharacters => {
                *screen = AppScreen::CharacterSelect;
                dirty.0 = true;
                if runtime_state.account_id.is_some() {
                    send_client_message(&mut client, &ClientMessage::ListCharacters);
                }
            }
        }
    }
}

fn handle_text_input(
    mut reader: MessageReader<KeyboardInput>,
    screen: Res<AppScreen>,
    mut login: ResMut<LoginForm>,
    mut create_form: ResMut<CharacterCreateForm>,
    mut dirty: ResMut<UiDirty>,
) {
    let active_login = if *screen == AppScreen::Login {
        Some(login.focused)
    } else {
        None
    };
    let active_create = if *screen == AppScreen::CharacterSelect {
        Some(create_form.focused)
    } else {
        None
    };
    if active_login.is_none() && active_create.is_none() {
        return;
    }

    for event in reader.read() {
        if event.state != bevy::input::ButtonState::Pressed || event.repeat {
            continue;
        }

        match event.key_code {
            KeyCode::Backspace => {
                if let Some(field) = active_login {
                    let value = match field {
                        LoginField::Email => &mut login.email,
                        LoginField::Password => &mut login.password,
                    };
                    value.pop();
                    dirty.0 = true;
                } else if let Some(field) = active_create {
                    let value = match field {
                        CreateField::Name => &mut create_form.name,
                        CreateField::ClassSlug => &mut create_form.class_slug,
                    };
                    value.pop();
                    dirty.0 = true;
                }
            }
            KeyCode::Tab => {
                if let Some(field) = active_login {
                    login.focused = match field {
                        LoginField::Email => LoginField::Password,
                        LoginField::Password => LoginField::Email,
                    };
                    dirty.0 = true;
                } else if let Some(field) = active_create {
                    create_form.focused = match field {
                        CreateField::Name => CreateField::ClassSlug,
                        CreateField::ClassSlug => CreateField::Name,
                    };
                    dirty.0 = true;
                }
            }
            _ => {
                if let Some(text) = &event.text {
                    let filtered = text
                        .chars()
                        .filter(|ch| !ch.is_control())
                        .collect::<String>();
                    if filtered.is_empty() {
                        continue;
                    }
                    if let Some(field) = active_login {
                        let value = match field {
                            LoginField::Email => &mut login.email,
                            LoginField::Password => &mut login.password,
                        };
                        value.push_str(&filtered);
                        dirty.0 = true;
                    } else if let Some(field) = active_create {
                        let value = match field {
                            CreateField::Name => &mut create_form.name,
                            CreateField::ClassSlug => &mut create_form.class_slug,
                        };
                        value.push_str(&filtered);
                        dirty.0 = true;
                    }
                } else if matches!(event.logical_key, Key::Space) {
                    if let Some(field) = active_create {
                        let value = match field {
                            CreateField::Name => &mut create_form.name,
                            CreateField::ClassSlug => &mut create_form.class_slug,
                        };
                        value.push(' ');
                        dirty.0 = true;
                    }
                }
            }
        }
    }
}

fn rebuild_ui(
    mut commands: Commands,
    config: Res<ClientConfig>,
    screen: Res<AppScreen>,
    status: Res<StatusMessage>,
    login: Res<LoginForm>,
    create_form: Res<CharacterCreateForm>,
    characters: Res<CharacterListState>,
    runtime_state: Res<ClientRuntimeState>,
    mut dirty: ResMut<UiDirty>,
    roots: Query<Entity, With<ScreenRoot>>,
) {
    if !dirty.0 {
        return;
    }
    dirty.0 = false;

    for entity in &roots {
        commands.entity(entity).despawn();
    }

    let root = commands
        .spawn((
            ScreenRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(18.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.07, 0.08)),
        ))
        .id();

    commands.entity(root).with_children(|root| {
        root.spawn(plain_text("Aurelis Game Client", 28.0));
        root.spawn(plain_text(format!("Server: {}", config.server_addr), 14.0));
        if !status.0.is_empty() {
            root.spawn((
                Text::new(status.0.clone()),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgb(0.97, 0.72, 0.38)),
            ));
        }

        match *screen {
            AppScreen::Login => spawn_login_screen(root, &login),
            AppScreen::CharacterSelect => {
                spawn_character_select_screen(root, &create_form, &characters)
            }
            AppScreen::Viewer => {
                if let Some(snapshot) = runtime_state.character_snapshot.as_ref() {
                    spawn_viewer_screen(root, snapshot);
                } else {
                    root.spawn(plain_text("Waiting for character snapshot...", 18.0));
                }
            }
        }
    });
}

fn format_stats(snapshot: &CharacterSnapshotView) -> String {
    let stats = &snapshot.stats.final_combat_stats;
    let rewards = &snapshot.stats.final_reward_stats;
    format!(
        "Final Stats\n\nHP: {}\nMP: {}\nPhysical Atk: {}\nMagical Atk: {}\nPhysical Def: {}\nMagical Def: {}\nMove Spd: {}\nAtk Spd: {}\nCrit Chance: {}\nCrit Damage: {}\nDamage Reduction: {}\nAccuracy: {}\nPhysical Atk Lv: {}\nMagical Atk Lv: {}\n\nRewards\nXP Gain: {}\nDrop Rate: {}\nCredit Gain: {}",
        stats.core.hp,
        stats.core.mp,
        stats.core.physical_atk,
        stats.core.magical_atk,
        stats.core.physical_def,
        stats.core.magical_def,
        stats.core.move_spd,
        stats.core.atk_spd,
        stats.secondary.crit_chance,
        stats.secondary.crit_damage,
        stats.secondary.damage_reduction,
        stats.secondary.accuracy,
        stats.secondary.physical_attack_level,
        stats.secondary.magical_attack_level,
        rewards.experience_gain,
        rewards.drop_rate,
        rewards.credit_gain
    )
}

fn format_equipped(snapshot: &CharacterSnapshotView) -> String {
    let mut out = String::from("Equipped\n\n");
    for slot in &snapshot.equipped {
        let item = &slot.item;
        let _ = writeln!(out, "{}: {} +{}", slot.slot, item.name, item.refinement);
        if !item.additional_effects.is_empty() {
            let _ = writeln!(out, "  rolled:");
            for effect in &item.additional_effects {
                let _ = writeln!(out, "    {} {} {}", effect.stat, effect.kind, effect.value);
            }
        }
        if !item.socketed_gems.is_empty() {
            let _ = writeln!(out, "  gems:");
            for gem in &item.socketed_gems {
                let _ = writeln!(out, "    {}", gem.name);
                for effect in &gem.additional_effects {
                    let _ = writeln!(
                        out,
                        "      {} {} {}",
                        effect.stat, effect.kind, effect.value
                    );
                }
            }
        }
    }
    out
}

fn format_inventories(snapshot: &CharacterSnapshotView) -> String {
    let mut out = String::from("Inventories\n\n");
    for inventory in &snapshot.inventories {
        let _ = writeln!(
            out,
            "{} ({}/{})",
            inventory.inventory_type,
            inventory
                .slots
                .iter()
                .filter(|slot| slot.item.is_some())
                .count(),
            inventory.capacity
        );
        for slot in inventory.slots.iter().filter(|slot| slot.item.is_some()) {
            let item = slot.item.as_ref().expect("slot item should exist");
            let _ = writeln!(
                out,
                "  [{}] {} x{}",
                slot.slot_index, item.name, slot.quantity
            );
        }
        let _ = writeln!(out);
    }
    out
}

fn format_runtime(snapshot: &CharacterSnapshotView) -> String {
    let mut out = String::from("Runtime\n\n");
    let _ = writeln!(out, "Current HP: {}", snapshot.stats.current_hp);
    let _ = writeln!(out, "Current MP: {}", snapshot.stats.current_mp);
    let _ = writeln!(out, "Credits: {}", snapshot.credits);
    let _ = writeln!(out, "Experience: {}", snapshot.experience);
    let _ = writeln!(out);

    let _ = writeln!(out, "Active Buffs");
    if snapshot.stats.active_buffs.is_empty() {
        let _ = writeln!(out, "  none");
    } else {
        for buff in &snapshot.stats.active_buffs {
            let _ = writeln!(out, "  {} ({} ms)", buff.effect_slug, buff.remaining_ms);
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "Cooldowns");
    if snapshot.stats.skill_cooldowns.is_empty() {
        let _ = writeln!(out, "  none");
    } else {
        for cooldown in &snapshot.stats.skill_cooldowns {
            let _ = writeln!(
                out,
                "  {} ({} ms)",
                cooldown.skill_slug, cooldown.remaining_ms
            );
        }
    }
    out
}

fn send_client_message(client: &mut RenetClient, message: &ClientMessage) {
    let payload = bincode::serialize(message).expect("failed to serialize client message");
    client.send_message(DefaultChannel::ReliableOrdered, payload);
}

fn log_disconnected(
    mut exit: MessageWriter<AppExit>,
    boot_state: Res<ClientBootState>,
    client: Option<Res<RenetClient>>,
) {
    if !boot_state.connected_once {
        return;
    }

    let Some(client) = client else {
        return;
    };

    if client.is_disconnected() {
        println!("game-client disconnected");
        exit.write(AppExit::Success);
    }
}

fn handle_netcode_error(netcode_error: On<NetcodeErrorEvent>) {
    eprintln!("game-client netcode error: {}", **netcode_error);
}
