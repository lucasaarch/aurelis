use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::quests)]
pub struct QuestModel {
    pub id: Uuid,
    pub slug: String,
    pub created_at: NaiveDateTime,
}

impl QuestModel {
    pub fn new(slug: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug,
            created_at: Utc::now().naive_utc(),
        }
    }
}
