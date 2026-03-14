use std::collections::HashMap;

use bevy::prelude::Resource;
use uuid::Uuid;

#[derive(Debug, Default, Resource)]
pub struct ClientSessions {
    pub by_client_id: HashMap<u64, ClientSession>,
}

#[derive(Debug, Clone)]
pub struct ClientSession {
    pub state: SessionState,
}

#[derive(Debug, Clone)]
pub enum SessionState {
    ConnectedUnauthenticated,
    Authenticated {
        account_id: Uuid,
    },
    CharacterSelected {
        account_id: Uuid,
        character_id: Uuid,
    },
}
