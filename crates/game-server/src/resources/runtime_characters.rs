use std::collections::HashMap;

use bevy::prelude::Resource;

use crate::runtime::character::RuntimeCharacter;

#[derive(Default, Resource)]
pub struct RuntimeCharacters {
    pub by_client_id: HashMap<u64, RuntimeCharacter>,
}
