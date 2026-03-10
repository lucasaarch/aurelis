use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyOrigin {
    Purchase,
    Trade,
    Bonus,
    Dungeon,
    Quest,
    Npc,
}
