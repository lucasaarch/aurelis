use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const RELIABLE_GAME_CHANNEL_ID: u8 = 0;

pub fn protocol_id_from_version(version: &str) -> u64 {
    let (core, prerelease) = version.split_once('-').unwrap_or((version, ""));
    let mut parts = core.split('.');

    let major = parts
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .expect("invalid major version for protocol id");
    let minor = parts
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .expect("invalid minor version for protocol id");
    let patch = parts
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .expect("invalid patch version for protocol id");

    let prerelease_iteration = prerelease
        .rsplit('.')
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0);

    (major << 48) | (minor << 32) | (patch << 16) | prerelease_iteration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Authenticate { token: String },
    SelectCharacter { character_id: Uuid },
    UseItem { inventory_type: String, slot: i16 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Authenticated { account_id: Uuid },
    AuthenticationFailed { reason: String },
    CharacterSelected { character_id: Uuid },
    CharacterSelectionFailed { reason: String },
    ItemUsed { inventory_type: String, slot: i16 },
    ItemUseFailed { reason: String },
}
