use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

use crate::models::character_class::CharacterClassModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::skills)]
pub struct SkillModel {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub character_class: CharacterClassModel,
    pub line_id: Option<Uuid>,
    pub level_req: i16,
    pub max_level: i16,
    pub created_at: NaiveDateTime,
}

impl SkillModel {
    pub fn new(
        slug: String,
        name: String,
        description: Option<String>,
        character_class: CharacterClassModel,
        line_id: Option<Uuid>,
        level_req: i16,
        max_level: i16,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug,
            name,
            description,
            character_class,
            line_id,
            level_req,
            max_level,
            created_at: Utc::now().naive_utc(),
        }
    }
}
