pub mod auth {
    tonic::include_proto!("auth");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");
}
