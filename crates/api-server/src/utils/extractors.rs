use tonic::{Status, metadata::MetadataMap};

pub fn extract_access_token_from_metadata(metadata: &MetadataMap) -> Result<String, Status> {
    let token = metadata
        .get("authorization")
        .ok_or(Status::unauthenticated("Missing Access Token"))?
        .to_str()
        .map_err(|_| Status::unauthenticated("Invalid Access Token"))?
        .trim_start_matches("Bearer ")
        .to_string();

    Ok(token)
}
