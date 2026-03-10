use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl ItemRarity {
    pub fn to_string(&self) -> String {
        match self {
            ItemRarity::Common => "common".to_string(),
            ItemRarity::Uncommon => "uncommon".to_string(),
            ItemRarity::Rare => "rare".to_string(),
            ItemRarity::Epic => "epic".to_string(),
        }
    }
}
