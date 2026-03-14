use bevy::prelude::Message;

#[derive(Debug, Message)]
pub struct PlayerConnected {
    pub client_id: u64,
}

#[derive(Debug, Message)]
pub struct PlayerDisconnected {
    pub client_id: u64,
    pub reason: String,
}
