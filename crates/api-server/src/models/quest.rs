use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

use crate::models::character_location::CharacterLocationModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::quests)]
pub struct QuestModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub city: Option<CharacterLocationModel>,
    pub level_req: i16,
    pub created_at: NaiveDateTime,
}

impl QuestModel {
    pub fn new(
        name: String,
        description: Option<String>,
        city: Option<CharacterLocationModel>,
        level_req: i16,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            city,
            level_req,
            created_at: Utc::now().naive_utc(),
        }
    }
}
