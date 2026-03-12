use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::CharacterClass)]
pub enum CharacterClassModel {
    Kael,
    Rin,
    Sirena,
}

impl ToSql<crate::db::schema::sql_types::CharacterClass, Pg> for CharacterClassModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            CharacterClassModel::Kael => out.write_all(b"kael")?,
            CharacterClassModel::Rin => out.write_all(b"rin")?,
            CharacterClassModel::Sirena => out.write_all(b"sirena")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::CharacterClass, Pg> for CharacterClassModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"kael" => Ok(CharacterClassModel::Kael),
            b"rin" => Ok(CharacterClassModel::Rin),
            b"sirena" => Ok(CharacterClassModel::Sirena),
            _ => Err("Unrecognized CharacterClass".into()),
        }
    }
}

impl std::fmt::Display for CharacterClassModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CharacterClassModel::Kael => "kael",
            CharacterClassModel::Rin => "rin",
            CharacterClassModel::Sirena => "sirena",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for CharacterClassModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kael" => Ok(CharacterClassModel::Kael),
            "rin" => Ok(CharacterClassModel::Rin),
            "sirena" => Ok(CharacterClassModel::Sirena),
            _ => Err(()),
        }
    }
}
