pub mod auth {
    tonic::include_proto!("auth");
}

pub mod character {
    tonic::include_proto!("character");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("protos_descriptor");