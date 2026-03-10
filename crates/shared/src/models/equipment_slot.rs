use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentSlot {
    Weapon,
    Head,
    Chest,
    Legs,
    Accessory,
}
