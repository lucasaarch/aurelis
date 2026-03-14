use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde_json::Value;
use uuid::Uuid;

use crate::models::quest_status::QuestStatusModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::player_account_quests)]
pub struct PlayerAccountQuestModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub quest_id: Uuid,
    pub completed_by_character_id: Option<Uuid>,
    pub status: QuestStatusModel,
    pub progress: Value,
    pub selected_reward_item_slug: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub claimed_at: Option<NaiveDateTime>,
}
