use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use shared::models::quest_status::QuestStatus;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::QuestStatus)]
pub enum QuestStatusModel {
    Available,
    InProgress,
    Completed,
}

impl ToSql<crate::db::schema::sql_types::QuestStatus, Pg> for QuestStatusModel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            QuestStatusModel::Available => out.write_all(b"available")?,
            QuestStatusModel::InProgress => out.write_all(b"in_progress")?,
            QuestStatusModel::Completed => out.write_all(b"completed")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::schema::sql_types::QuestStatus, Pg> for QuestStatusModel {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"available" => Ok(QuestStatusModel::Available),
            b"in_progress" => Ok(QuestStatusModel::InProgress),
            b"completed" => Ok(QuestStatusModel::Completed),
            _ => Err("Unrecognized QuestStatus".into()),
        }
    }
}

impl From<QuestStatusModel> for QuestStatus {
    fn from(model: QuestStatusModel) -> Self {
        match model {
            QuestStatusModel::Available => QuestStatus::Available,
            QuestStatusModel::InProgress => QuestStatus::InProgress,
            QuestStatusModel::Completed => QuestStatus::Completed,
        }
    }
}

impl std::str::FromStr for QuestStatusModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "available" => Ok(QuestStatusModel::Available),
            "in_progress" => Ok(QuestStatusModel::InProgress),
            "completed" => Ok(QuestStatusModel::Completed),
            _ => Err(()),
        }
    }
}
