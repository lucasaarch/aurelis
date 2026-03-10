use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSkill {
    pub character_id: Uuid,
    pub skill_id: Uuid,
    pub unlocked_at: NaiveDateTime,
}
