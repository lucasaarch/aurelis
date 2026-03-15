use shared::models::{
    combat_stats::CombatStats, reward_stats::RewardStats, stat_modifier::ModifierStat,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RuntimeModifier {
    pub source: ModifierSource,
    pub duration: ModifierDuration,
    pub operations: Vec<StatModifierOp>,
}

#[derive(Debug, Clone)]
pub enum ModifierSource {
    PassiveSkill {
        skill_slug: String,
    },
    ActiveBuff {
        effect_slug: String,
    },
    Equipment {
        item_instance_id: Uuid,
    },
    Gem {
        item_instance_id: Uuid,
        slot_index: i16,
    },
    Consumable {
        item_slug: String,
    },
    Debug {
        label: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierDuration {
    Permanent,
    Timed { remaining_ms: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatModifierOp {
    AddFlat { stat: ModifierStat, value: i32 },
    AddPercent { stat: ModifierStat, value_bp: i32 },
}

impl RuntimeModifier {
    pub fn apply_to_stats(&self, combat_stats: &mut CombatStats, reward_stats: &mut RewardStats) {
        for operation in &self.operations {
            match *operation {
                StatModifierOp::AddFlat { stat, value } => {
                    apply_flat(combat_stats, reward_stats, stat, value)
                }
                StatModifierOp::AddPercent { stat, value_bp } => {
                    apply_percent(combat_stats, reward_stats, stat, value_bp)
                }
            }
        }
    }

    pub fn tick(&mut self, elapsed_ms: u64) -> bool {
        match &mut self.duration {
            ModifierDuration::Permanent => false,
            ModifierDuration::Timed { remaining_ms } => {
                *remaining_ms = remaining_ms.saturating_sub(elapsed_ms);
                *remaining_ms == 0
            }
        }
    }
}

fn apply_flat(
    combat_stats: &mut CombatStats,
    reward_stats: &mut RewardStats,
    stat: ModifierStat,
    value: i32,
) {
    match stat {
        ModifierStat::Combat(stat) => combat_stats.add_to_stat(stat, value),
        ModifierStat::Reward(stat) => reward_stats.add_to_stat(stat, value),
    }
}

fn apply_percent(
    combat_stats: &mut CombatStats,
    reward_stats: &mut RewardStats,
    stat: ModifierStat,
    value_bp: i32,
) {
    match stat {
        ModifierStat::Combat(stat) => combat_stats.add_percent_to_stat(stat, value_bp),
        ModifierStat::Reward(stat) => reward_stats.add_percent_to_stat(stat, value_bp),
    }
}
