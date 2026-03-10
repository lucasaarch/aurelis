use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSkillSlot {
    pub character_id: Uuid,
    pub slot: i16,
    pub skill_id: Uuid,
}
