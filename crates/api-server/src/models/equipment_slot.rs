use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use shared::models::equipment_slot::EquipmentSlot;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::EquipmentSlotType)]
pub enum EquipmentSlotModel {
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

impl ToSql<crate::db::schema::sql_types::EquipmentSlotType, Pg> for EquipmentSlotModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            EquipmentSlotModel::Weapon => out.write_all(b"weapon")?,
            EquipmentSlotModel::Head => out.write_all(b"head")?,
            EquipmentSlotModel::Chest => out.write_all(b"chest")?,
            EquipmentSlotModel::Legs => out.write_all(b"legs")?,
            EquipmentSlotModel::Gloves => out.write_all(b"gloves")?,
            EquipmentSlotModel::Shoes => out.write_all(b"shoes")?,
            EquipmentSlotModel::AccRing1 => out.write_all(b"acc_ring_1")?,
            EquipmentSlotModel::AccRing2 => out.write_all(b"acc_ring_2")?,
            EquipmentSlotModel::AccNecklace => out.write_all(b"acc_necklace")?,
            EquipmentSlotModel::AccEarrings => out.write_all(b"acc_earrings")?,
            EquipmentSlotModel::AccArm => out.write_all(b"acc_arm")?,
            EquipmentSlotModel::AccFaceBottom => out.write_all(b"acc_face_bottom")?,
            EquipmentSlotModel::AccFaceMiddle => out.write_all(b"acc_face_middle")?,
            EquipmentSlotModel::AccFaceTop => out.write_all(b"acc_face_top")?,
            EquipmentSlotModel::AccBottomPiece => out.write_all(b"acc_bottom_piece")?,
            EquipmentSlotModel::AccTopPiece => out.write_all(b"acc_top_piece")?,
            EquipmentSlotModel::AccWeapon => out.write_all(b"acc_weapon")?,
            EquipmentSlotModel::AccSupportUnit => out.write_all(b"acc_support_unit")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::EquipmentSlotType, Pg> for EquipmentSlotModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"weapon" => Ok(EquipmentSlotModel::Weapon),
            b"head" => Ok(EquipmentSlotModel::Head),
            b"chest" => Ok(EquipmentSlotModel::Chest),
            b"legs" => Ok(EquipmentSlotModel::Legs),
            b"gloves" => Ok(EquipmentSlotModel::Gloves),
            b"shoes" => Ok(EquipmentSlotModel::Shoes),
            b"acc_ring_1" => Ok(EquipmentSlotModel::AccRing1),
            b"acc_ring_2" => Ok(EquipmentSlotModel::AccRing2),
            b"acc_necklace" => Ok(EquipmentSlotModel::AccNecklace),
            b"acc_earrings" => Ok(EquipmentSlotModel::AccEarrings),
            b"acc_arm" => Ok(EquipmentSlotModel::AccArm),
            b"acc_face_bottom" => Ok(EquipmentSlotModel::AccFaceBottom),
            b"acc_face_middle" => Ok(EquipmentSlotModel::AccFaceMiddle),
            b"acc_face_top" => Ok(EquipmentSlotModel::AccFaceTop),
            b"acc_bottom_piece" => Ok(EquipmentSlotModel::AccBottomPiece),
            b"acc_top_piece" => Ok(EquipmentSlotModel::AccTopPiece),
            b"acc_weapon" => Ok(EquipmentSlotModel::AccWeapon),
            b"acc_support_unit" => Ok(EquipmentSlotModel::AccSupportUnit),
            _ => Err("Unrecognized EquipmentSlot".into()),
        }
    }
}

impl From<EquipmentSlotModel> for EquipmentSlot {
    fn from(model: EquipmentSlotModel) -> Self {
        match model {
            EquipmentSlotModel::Weapon => EquipmentSlot::Weapon,
            EquipmentSlotModel::Head => EquipmentSlot::Head,
            EquipmentSlotModel::Chest => EquipmentSlot::Chest,
            EquipmentSlotModel::Legs => EquipmentSlot::Legs,
            EquipmentSlotModel::Gloves => EquipmentSlot::Gloves,
            EquipmentSlotModel::Shoes => EquipmentSlot::Shoes,
            EquipmentSlotModel::AccRing1 => EquipmentSlot::AccRing1,
            EquipmentSlotModel::AccRing2 => EquipmentSlot::AccRing2,
            EquipmentSlotModel::AccNecklace => EquipmentSlot::AccNecklace,
            EquipmentSlotModel::AccEarrings => EquipmentSlot::AccEarrings,
            EquipmentSlotModel::AccArm => EquipmentSlot::AccArm,
            EquipmentSlotModel::AccFaceBottom => EquipmentSlot::AccFaceBottom,
            EquipmentSlotModel::AccFaceMiddle => EquipmentSlot::AccFaceMiddle,
            EquipmentSlotModel::AccFaceTop => EquipmentSlot::AccFaceTop,
            EquipmentSlotModel::AccBottomPiece => EquipmentSlot::AccBottomPiece,
            EquipmentSlotModel::AccTopPiece => EquipmentSlot::AccTopPiece,
            EquipmentSlotModel::AccWeapon => EquipmentSlot::AccWeapon,
            EquipmentSlotModel::AccSupportUnit => EquipmentSlot::AccSupportUnit,
        }
    }
}

impl std::str::FromStr for EquipmentSlotModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "weapon" => Ok(EquipmentSlotModel::Weapon),
            "head" => Ok(EquipmentSlotModel::Head),
            "chest" => Ok(EquipmentSlotModel::Chest),
            "legs" => Ok(EquipmentSlotModel::Legs),
            "gloves" => Ok(EquipmentSlotModel::Gloves),
            "shoes" => Ok(EquipmentSlotModel::Shoes),
            "acc_ring_1" => Ok(EquipmentSlotModel::AccRing1),
            "acc_ring_2" => Ok(EquipmentSlotModel::AccRing2),
            "acc_necklace" => Ok(EquipmentSlotModel::AccNecklace),
            "acc_earrings" => Ok(EquipmentSlotModel::AccEarrings),
            "acc_arm" => Ok(EquipmentSlotModel::AccArm),
            "acc_face_bottom" => Ok(EquipmentSlotModel::AccFaceBottom),
            "acc_face_middle" => Ok(EquipmentSlotModel::AccFaceMiddle),
            "acc_face_top" => Ok(EquipmentSlotModel::AccFaceTop),
            "acc_bottom_piece" => Ok(EquipmentSlotModel::AccBottomPiece),
            "acc_top_piece" => Ok(EquipmentSlotModel::AccTopPiece),
            "acc_weapon" => Ok(EquipmentSlotModel::AccWeapon),
            "acc_support_unit" => Ok(EquipmentSlotModel::AccSupportUnit),
            _ => Err(()),
        }
    }
}
