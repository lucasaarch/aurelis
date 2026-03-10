use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentSlot {
    Weapon,
    Head,
    Chest,
    Legs,
    Gloves,
    Shoes,
    AccRing1,
    AccRing2,
    AccNecklace,
    AccEarrings,
    AccArm,
    AccFaceBottom,
    AccFaceMiddle,
    AccFaceTop,
    AccBottomPiece,
    AccTopPiece,
    AccWeapon,
    AccSupportUnit,
}
