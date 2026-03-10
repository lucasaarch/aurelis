use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}
