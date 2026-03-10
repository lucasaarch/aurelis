use bigdecimal::BigDecimal;
use uuid::Uuid;
use validator::ValidationError;

pub fn validate_rarity(value: &str) -> Result<(), ValidationError> {
    match value {
        "common" | "uncommon" | "rare" | "epic" => Ok(()),
        _ => Err(ValidationError::new("invalid_rarity")),
    }
}

pub fn validate_equipment_slot(value: &str) -> Result<(), ValidationError> {
    match value {
        "weapon" | "head" | "chest" | "legs" | "gloves" | "shoes" | "acc_ring_1" | "acc_ring_2"
        | "acc_necklace" | "acc_earrings" | "acc_arm" | "acc_face_bottom" | "acc_face_middle"
        | "acc_face_top" | "acc_bottom_piece" | "acc_top_piece" | "acc_weapon"
        | "acc_support_unit" => Ok(()),
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

pub fn validate_inventory_type(value: &str) -> Result<(), ValidationError> {
    match value {
        "equipment" | "accessory" | "consumable" | "material" | "quest_item" | "special" => Ok(()),
        _ => Err(ValidationError::new("invalid_inventory_type")),
    }
}

pub fn validate_drop_chance(value: &BigDecimal) -> Result<(), ValidationError> {
    use bigdecimal::ToPrimitive;

    if let Some(f) = value.to_f64()
        && f > 0.0
        && f <= 100.0
    {
        return Ok(());
    }

    let mut err = ValidationError::new("drop_chance");
    err.message = Some("must be > 0 and <= 100".into());
    Err(err)
}

pub fn validate_uuid(value: &Uuid) -> Result<(), ValidationError> {
    match uuid::Uuid::parse_str(&value.to_string()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_uuid")),
    }
}

pub fn validate_strong_password(password: &str) -> Result<(), ValidationError> {
    let checks = [
        (
            password.chars().any(|c| c.is_uppercase()),
            "must contain at least one uppercase letter",
        ),
        (
            password.chars().any(|c| c.is_lowercase()),
            "must contain at least one lowercase letter",
        ),
        (
            password.chars().any(|c| c.is_ascii_digit()),
            "must contain at least one digit",
        ),
        (
            password.chars().any(|c| !c.is_alphanumeric()),
            "must contain at least one special character",
        ),
    ];

    for (valid, msg) in checks {
        if !valid {
            let mut err = ValidationError::new("strong_password");
            err.message = Some(msg.into());
            return Err(err);
        }
    }

    Ok(())
}
