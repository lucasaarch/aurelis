use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use shared::models::character_quest::CharacterQuest;
use uuid::Uuid;

use crate::models::quest_status::QuestStatusModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::character_quests)]
pub struct CharacterQuestModel {
    pub id: Uuid,
    pub character_id: Uuid,
    pub quest_id: Uuid,
    pub status: QuestStatusModel,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
}

impl CharacterQuestModel {
    pub fn new(character_id: Uuid, quest_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            character_id,
            quest_id,
            status: QuestStatusModel::Available,
            started_at: None,
            completed_at: None,
        }
    }
}

impl From<CharacterQuestModel> for CharacterQuest {
    fn from(model: CharacterQuestModel) -> Self {
        Self {
            id: model.id,
            character_id: model.character_id,
            quest_id: model.quest_id,
            status: model.status.into(),
            started_at: model.started_at,
            completed_at: model.completed_at,
        }
    }
}
