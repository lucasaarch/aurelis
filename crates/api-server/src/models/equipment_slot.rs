use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, AsExpression, FromSqlRow)]
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

impl std::fmt::Display for EquipmentSlotModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EquipmentSlotModel::Weapon => "weapon",
            EquipmentSlotModel::Head => "head",
            EquipmentSlotModel::Chest => "chest",
            EquipmentSlotModel::Legs => "legs",
            EquipmentSlotModel::Gloves => "gloves",
            EquipmentSlotModel::Shoes => "shoes",
            EquipmentSlotModel::AccRing1 => "acc_ring_1",
            EquipmentSlotModel::AccRing2 => "acc_ring_2",
            EquipmentSlotModel::AccNecklace => "acc_necklace",
            EquipmentSlotModel::AccEarrings => "acc_earrings",
            EquipmentSlotModel::AccArm => "acc_arm",
            EquipmentSlotModel::AccFaceBottom => "acc_face_bottom",
            EquipmentSlotModel::AccFaceMiddle => "acc_face_middle",
            EquipmentSlotModel::AccFaceTop => "acc_face_top",
            EquipmentSlotModel::AccBottomPiece => "acc_bottom_piece",
            EquipmentSlotModel::AccTopPiece => "acc_top_piece",
            EquipmentSlotModel::AccWeapon => "acc_weapon",
            EquipmentSlotModel::AccSupportUnit => "acc_support_unit",
        };
        f.write_str(value)
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
