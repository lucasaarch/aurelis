use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use shared::models::mob_type::MobType;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::MobType)]
pub enum MobTypeModel {
    Common,
    Miniboss,
    Boss,
    RaidBoss,
}

impl ToSql<crate::db::schema::sql_types::MobType, Pg> for MobTypeModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            MobTypeModel::Common => out.write_all(b"common")?,
            MobTypeModel::Miniboss => out.write_all(b"miniboss")?,
            MobTypeModel::Boss => out.write_all(b"boss")?,
            MobTypeModel::RaidBoss => out.write_all(b"raidboss")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::MobType, Pg> for MobTypeModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"common" => Ok(MobTypeModel::Common),
            b"miniboss" => Ok(MobTypeModel::Miniboss),
            b"boss" => Ok(MobTypeModel::Boss),
            b"raidboss" => Ok(MobTypeModel::RaidBoss),
            _ => Err("Unrecognized MobType".into()),
        }
    }
}

impl From<MobTypeModel> for MobType {
    fn from(model: MobTypeModel) -> Self {
        match model {
            MobTypeModel::Common => MobType::Common,
            MobTypeModel::Miniboss => MobType::Miniboss,
            MobTypeModel::Boss => MobType::Boss,
            MobTypeModel::RaidBoss => MobType::RaidBoss,
        }
    }
}

impl std::str::FromStr for MobTypeModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "common" => Ok(MobTypeModel::Common),
            "miniboss" => Ok(MobTypeModel::Miniboss),
            "boss" => Ok(MobTypeModel::Boss),
            "raidboss" => Ok(MobTypeModel::RaidBoss),
            _ => Err(()),
        }
    }
}
