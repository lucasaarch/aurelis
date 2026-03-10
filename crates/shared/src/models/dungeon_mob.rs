use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DungeonMob {
    pub id: Uuid,
    pub mob_id: Uuid,
    pub dungeon_id: String,
}
