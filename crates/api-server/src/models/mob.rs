use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::mobs)]
pub struct MobModel {
    pub id: Uuid,
    pub slug: String,
    pub created_at: NaiveDateTime,
}

impl MobModel {
    pub fn new(slug: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug,
            created_at: Utc::now().naive_utc(),
        }
    }
}
