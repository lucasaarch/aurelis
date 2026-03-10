use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use shared::models::character_location::CharacterLocation;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::CharacterLocation)]
pub enum CharacterLocationModel {
    Aurelis,
    Volcanis,
    Aquavale,
    Sylvandar,
}

impl ToSql<crate::db::schema::sql_types::CharacterLocation, Pg> for CharacterLocationModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            CharacterLocationModel::Aurelis => out.write_all(b"aurelis")?,
            CharacterLocationModel::Volcanis => out.write_all(b"volcanis")?,
            CharacterLocationModel::Aquavale => out.write_all(b"aquavale")?,
            CharacterLocationModel::Sylvandar => out.write_all(b"sylvandar")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::CharacterLocation, Pg> for CharacterLocationModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"aurelis" => Ok(CharacterLocationModel::Aurelis),
            b"volcanis" => Ok(CharacterLocationModel::Volcanis),
            b"aquavale" => Ok(CharacterLocationModel::Aquavale),
            b"sylvandar" => Ok(CharacterLocationModel::Sylvandar),
            _ => Err("Unrecognized CharacterLocation".into()),
        }
    }
}

impl From<CharacterLocationModel> for CharacterLocation {
    fn from(model: CharacterLocationModel) -> Self {
        match model {
            CharacterLocationModel::Aurelis => CharacterLocation::Aurelis,
            CharacterLocationModel::Volcanis => CharacterLocation::Volcanis,
            CharacterLocationModel::Aquavale => CharacterLocation::Aquavale,
            CharacterLocationModel::Sylvandar => CharacterLocation::Sylvandar,
        }
    }
}