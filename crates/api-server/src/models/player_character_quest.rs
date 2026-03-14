use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde_json::Value;
use uuid::Uuid;

use crate::models::quest_status::QuestStatusModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::player_character_quests)]
pub struct PlayerCharacterQuestModel {
    pub id: Uuid,
    pub character_id: Uuid,
    pub quest_id: Uuid,
    pub status: QuestStatusModel,
    pub progress: Value,
    pub selected_reward_item_slug: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub claimed_at: Option<NaiveDateTime>,
}
