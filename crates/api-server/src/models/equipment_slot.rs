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
    Accessory,
}

impl ToSql<crate::db::schema::sql_types::EquipmentSlotType, Pg> for EquipmentSlotModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            EquipmentSlotModel::Weapon => out.write_all(b"weapon")?,
            EquipmentSlotModel::Head => out.write_all(b"head")?,
            EquipmentSlotModel::Chest => out.write_all(b"chest")?,
            EquipmentSlotModel::Legs => out.write_all(b"legs")?,
            EquipmentSlotModel::Accessory => out.write_all(b"accessory")?,
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
            b"accessory" => Ok(EquipmentSlotModel::Accessory),
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
            EquipmentSlotModel::Accessory => EquipmentSlot::Accessory,
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
            "accessory" => Ok(EquipmentSlotModel::Accessory),
            _ => Err(()),
        }
    }
}
