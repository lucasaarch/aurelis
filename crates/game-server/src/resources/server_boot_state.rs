use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct ServerBootState {
    pub announced: bool,
}
