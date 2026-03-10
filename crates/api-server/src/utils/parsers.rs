use uuid::Uuid;

use crate::error::AppError;

pub fn parse_uuid(uuid: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(uuid).map_err(|e| AppError::BadRequest(format!("Invalid UUID: {}", e)))
}
