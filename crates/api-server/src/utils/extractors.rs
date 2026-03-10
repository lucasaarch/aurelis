use tonic::metadata::MetadataMap;

use crate::error::AppError;

pub fn extract_access_token_from_metadata(metadata: &MetadataMap) -> Result<String, AppError> {
    let token = metadata
        .get("authorization")
        .ok_or(AppError::Unauthorized("Missing Access Token".to_string()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Access Token".to_string()))?
        .trim_start_matches("Bearer ")
        .to_string();

    Ok(token)
}
