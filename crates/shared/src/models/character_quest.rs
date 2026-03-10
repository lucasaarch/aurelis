use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::quest_status::QuestStatus;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterQuest {
    pub id: Uuid,
    pub character_id: Uuid,
    pub quest_id: Uuid,
    pub status: QuestStatus,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
}
