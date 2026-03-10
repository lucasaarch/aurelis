use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use shared::models::currency_origin::CurrencyOrigin;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::CurrencyOrigin)]
pub enum CurrencyOriginModel {
    Purchase,
    Trade,
    Bonus,
    Dungeon,
    Quest,
    Npc,
}

impl ToSql<crate::db::schema::sql_types::CurrencyOrigin, Pg> for CurrencyOriginModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            CurrencyOriginModel::Purchase => out.write_all(b"purchase")?,
            CurrencyOriginModel::Trade => out.write_all(b"trade")?,
            CurrencyOriginModel::Bonus => out.write_all(b"bonus")?,
            CurrencyOriginModel::Dungeon => out.write_all(b"dungeon")?,
            CurrencyOriginModel::Quest => out.write_all(b"quest")?,
            CurrencyOriginModel::Npc => out.write_all(b"npc")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::CurrencyOrigin, Pg> for CurrencyOriginModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"purchase" => Ok(CurrencyOriginModel::Purchase),
            b"trade" => Ok(CurrencyOriginModel::Trade),
            b"bonus" => Ok(CurrencyOriginModel::Bonus),
            b"dungeon" => Ok(CurrencyOriginModel::Dungeon),
            b"quest" => Ok(CurrencyOriginModel::Quest),
            b"npc" => Ok(CurrencyOriginModel::Npc),
            _ => Err("Unrecognized CurrencyOrigin".into()),
        }
    }
}

impl From<CurrencyOriginModel> for CurrencyOrigin {
    fn from(model: CurrencyOriginModel) -> Self {
        match model {
            CurrencyOriginModel::Purchase => CurrencyOrigin::Purchase,
            CurrencyOriginModel::Trade => CurrencyOrigin::Trade,
            CurrencyOriginModel::Bonus => CurrencyOrigin::Bonus,
            CurrencyOriginModel::Dungeon => CurrencyOrigin::Dungeon,
            CurrencyOriginModel::Quest => CurrencyOrigin::Quest,
            CurrencyOriginModel::Npc => CurrencyOrigin::Npc,
        }
    }
}

impl std::str::FromStr for CurrencyOriginModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "purchase" => Ok(CurrencyOriginModel::Purchase),
            "trade" => Ok(CurrencyOriginModel::Trade),
            "bonus" => Ok(CurrencyOriginModel::Bonus),
            "dungeon" => Ok(CurrencyOriginModel::Dungeon),
            "quest" => Ok(CurrencyOriginModel::Quest),
            "npc" => Ok(CurrencyOriginModel::Npc),
            _ => Err(()),
        }
    }
}
