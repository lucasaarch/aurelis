use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::character_class::CharacterClass;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub character_class: CharacterClass,
    pub line_id: Option<Uuid>,
    pub level_req: i16,
    pub max_level: i16,
    pub created_at: NaiveDateTime,
}
