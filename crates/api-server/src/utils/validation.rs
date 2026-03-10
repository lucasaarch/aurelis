use validator::ValidationError;

pub fn validate_rarity(value: &str) -> Result<(), ValidationError> {
    match value {
        "common" | "uncommon" | "rare" | "epic" => Ok(()),
        _ => Err(ValidationError::new("invalid_rarity")),
    }
}

pub fn validate_equipment_slot(value: &str) -> Result<(), ValidationError> {
    match value {
        "weapon" | "head" | "chest" | "legs" | "accessory" => Ok(()),
        _ => Err(ValidationError::new("invalid_equipment_slot")),
    }
}

pub fn validate_class(value: &str) -> Result<(), ValidationError> {
    match value {
        "kael" | "rin" | "sirena" => Ok(()),
        _ => Err(ValidationError::new("invalid_class")),
    }
}

pub fn validate_stats(value: &serde_json::Value) -> Result<(), ValidationError> {
    if value.is_object() {
        Ok(())
    } else {
        Err(ValidationError::new("stats_must_be_object"))
    }
}

pub fn validate_mob_type(value: &str) -> Result<(), ValidationError> {
    match value {
        "common" | "miniboss" | "boss" | "raidboss" => Ok(()),
        _ => Err(ValidationError::new("invalid_mob_type")),
    }
}