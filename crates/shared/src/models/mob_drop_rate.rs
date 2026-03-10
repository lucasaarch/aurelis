use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobDropRate {
    pub id: Uuid,
    pub mob_id: Uuid,
    pub item_id: Uuid,
    pub drop_chance: BigDecimal,
}
