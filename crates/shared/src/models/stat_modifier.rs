use serde::{Deserialize, Serialize};

use crate::models::{combat_stats::StatKey, reward_stats::RewardStatKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifierStat {
    Combat(StatKey),
    Reward(RewardStatKey),
}
