use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::InventoryType)]
pub enum InventoryTypeModel {
    Equipment,
    Accessory,
    Consumable,
    Material,
    QuestItem,
    Special,
}

impl ToSql<crate::db::schema::sql_types::InventoryType, Pg> for InventoryTypeModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            InventoryTypeModel::Equipment => out.write_all(b"equipment")?,
            InventoryTypeModel::Accessory => out.write_all(b"accessory")?,
            InventoryTypeModel::Consumable => out.write_all(b"consumable")?,
            InventoryTypeModel::Material => out.write_all(b"material")?,
            InventoryTypeModel::QuestItem => out.write_all(b"quest_item")?,
            InventoryTypeModel::Special => out.write_all(b"special")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::InventoryType, Pg> for InventoryTypeModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"equipment" => Ok(InventoryTypeModel::Equipment),
            b"accessory" => Ok(InventoryTypeModel::Accessory),
            b"consumable" => Ok(InventoryTypeModel::Consumable),
            b"material" => Ok(InventoryTypeModel::Material),
            b"quest_item" => Ok(InventoryTypeModel::QuestItem),
            b"special" => Ok(InventoryTypeModel::Special),
            _ => Err("Unrecognized InventoryType".into()),
        }
    }
}

impl std::fmt::Display for InventoryTypeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            InventoryTypeModel::Equipment => "equipment",
            InventoryTypeModel::Accessory => "accessory",
            InventoryTypeModel::Consumable => "consumable",
            InventoryTypeModel::Material => "material",
            InventoryTypeModel::QuestItem => "quest_item",
            InventoryTypeModel::Special => "special",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for InventoryTypeModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "equipment" => Ok(InventoryTypeModel::Equipment),
            "accessory" => Ok(InventoryTypeModel::Accessory),
            "consumable" => Ok(InventoryTypeModel::Consumable),
            "material" => Ok(InventoryTypeModel::Material),
            "quest_item" => Ok(InventoryTypeModel::QuestItem),
            "special" => Ok(InventoryTypeModel::Special),
            _ => Err(()),
        }
    }
}
