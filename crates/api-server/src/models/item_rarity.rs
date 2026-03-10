use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use shared::models::item_rarity::ItemRarity;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::ItemRarity)]
pub enum ItemRarityModel {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl ToSql<crate::db::schema::sql_types::ItemRarity, Pg> for ItemRarityModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            ItemRarityModel::Common => out.write_all(b"common")?,
            ItemRarityModel::Uncommon => out.write_all(b"uncommon")?,
            ItemRarityModel::Rare => out.write_all(b"rare")?,
            ItemRarityModel::Epic => out.write_all(b"epic")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::ItemRarity, Pg> for ItemRarityModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"common" => Ok(ItemRarityModel::Common),
            b"uncommon" => Ok(ItemRarityModel::Uncommon),
            b"rare" => Ok(ItemRarityModel::Rare),
            b"epic" => Ok(ItemRarityModel::Epic),
            _ => Err("Unrecognized ItemRarity".into()),
        }
    }
}

impl From<ItemRarityModel> for ItemRarity {
    fn from(model: ItemRarityModel) -> Self {
        match model {
            ItemRarityModel::Common => ItemRarity::Common,
            ItemRarityModel::Uncommon => ItemRarity::Uncommon,
            ItemRarityModel::Rare => ItemRarity::Rare,
            ItemRarityModel::Epic => ItemRarity::Epic,
        }
    }
}

impl std::str::FromStr for ItemRarityModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "common" => Ok(ItemRarityModel::Common),
            "uncommon" => Ok(ItemRarityModel::Uncommon),
            "rare" => Ok(ItemRarityModel::Rare),
            "epic" => Ok(ItemRarityModel::Epic),
            _ => Err(()),
        }
    }
}
