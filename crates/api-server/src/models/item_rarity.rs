use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, AsExpression, FromSqlRow)]
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

impl std::fmt::Display for ItemRarityModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ItemRarityModel::Common => "common",
            ItemRarityModel::Uncommon => "uncommon",
            ItemRarityModel::Rare => "rare",
            ItemRarityModel::Epic => "epic",
        };
        f.write_str(value)
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
