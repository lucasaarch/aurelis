use serde::{Deserialize, Serialize};

use crate::models::stat_modifier::ModifierStat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatModifierValueKind {
    Flat,
    Percent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemInstanceStatModifier {
    pub id: String,
    pub stat: ModifierStat,
    pub kind: StatModifierValueKind,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentRollBias {
    Neutral,
    Physical,
    Magical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemInstanceAttributes {
    #[serde(default)]
    pub identified: bool,
    #[serde(default)]
    pub roll_bias: Option<EquipmentRollBias>,
    #[serde(default)]
    pub reroll_count: u16,
    #[serde(default)]
    pub additional_effects: Vec<ItemInstanceStatModifier>,
}

impl Default for ItemInstanceAttributes {
    fn default() -> Self {
        Self {
            identified: false,
            roll_bias: None,
            reroll_count: 0,
            additional_effects: vec![],
        }
    }
}
