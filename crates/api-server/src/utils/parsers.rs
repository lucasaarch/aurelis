use uuid::Uuid;

use crate::error::AppError;
use crate::models::suspension_severity::SuspensionSeverity;

pub fn parse_uuid(uuid: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(uuid).map_err(|e| AppError::BadRequest(format!("Invalid UUID: {}", e)))
}

pub fn parse_suspension_severity(value: &str) -> Result<SuspensionSeverity, AppError> {
    match value {
        "low" => Ok(SuspensionSeverity::Low),
        "medium" => Ok(SuspensionSeverity::Medium),
        "high" => Ok(SuspensionSeverity::High),
        _ => Err(AppError::BadRequest(
            "Invalid suspension severity".to_string(),
        )),
    }
}
