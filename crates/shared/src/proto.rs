pub mod auth {
    tonic::include_proto!("auth");
}

pub mod character {
    tonic::include_proto!("character");
}

pub mod inventory {
    tonic::include_proto!("inventory");
}

pub mod internal_game {
    tonic::include_proto!("internal_game");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("protos_descriptor");
