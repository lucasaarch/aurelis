use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
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

impl From<EquipmentSlot> for String {
    fn from(slot: EquipmentSlot) -> Self {
        match slot {
            EquipmentSlot::Weapon => "weapon",
            EquipmentSlot::Head => "head",
            EquipmentSlot::Chest => "chest",
            EquipmentSlot::Legs => "legs",
            EquipmentSlot::Gloves => "gloves",
            EquipmentSlot::Shoes => "shoes",
            EquipmentSlot::AccRing1 => "acc_ring_1",
            EquipmentSlot::AccRing2 => "acc_ring_2",
            EquipmentSlot::AccNecklace => "acc_necklace",
            EquipmentSlot::AccEarrings => "acc_earrings",
            EquipmentSlot::AccArm => "acc_arm",
            EquipmentSlot::AccFaceBottom => "acc_face_bottom",
            EquipmentSlot::AccFaceMiddle => "acc_face_middle",
            EquipmentSlot::AccFaceTop => "acc_face_top",
            EquipmentSlot::AccBottomPiece => "acc_bottom_piece",
            EquipmentSlot::AccTopPiece => "acc_top_piece",
            EquipmentSlot::AccWeapon => "acc_weapon",
            EquipmentSlot::AccSupportUnit => "acc_support_unit",
        }
        .to_string()
    }
}
