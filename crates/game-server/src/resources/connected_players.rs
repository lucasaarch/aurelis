use std::collections::HashSet;

use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct ConnectedPlayers {
    pub client_ids: HashSet<u64>,
}
