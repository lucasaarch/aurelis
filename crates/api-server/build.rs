fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .file_descriptor_set_path(
            std::path::PathBuf::from(std::env::var("OUT_DIR")?).join("protos_descriptor.bin"),
        )
        .compile_protos(
            &[
                "proto/auth.proto",
                "proto/character.proto",
                "proto/inventory.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
