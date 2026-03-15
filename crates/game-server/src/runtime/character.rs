use std::collections::HashMap;

use shared::models::{
    character_data::{CharacterSkillUnlocks, CombatAffinity},
    combat_stats::CombatStats,
    equipment_slot::EquipmentSlot,
    item_data::ItemData,
    reward_stats::RewardStats,
};
use uuid::Uuid;

use crate::runtime::modifier::RuntimeModifier;

#[allow(dead_code)]
#[derive(Clone)]
pub struct RuntimeCharacter {
    pub account_id: Uuid,
    pub character_id: Uuid,
    pub name: String,
    pub base_character_slug: String,
    pub current_class_slug: String,
    pub combat_affinity: CombatAffinity,
    pub level: i16,
    pub experience: i64,
    pub credits: i64,
    pub skill_unlocks: CharacterSkillUnlocks,
    pub available_skill_slugs: Vec<String>,
    pub loadout: RuntimeLoadout,
    pub skill_cooldowns_ms: HashMap<String, u64>,
    pub persistent_modifiers: Vec<RuntimeModifier>,
    pub timed_modifiers: Vec<RuntimeModifier>,
    pub resources: RuntimeResources,
    pub stats: RuntimeStatBlock,
    pub rewards: RuntimeRewardBlock,
}

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct RuntimeLoadout {
    pub equipped: HashMap<EquipmentSlot, ResolvedEquippedItem>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ResolvedEquippedItem {
    pub item_instance_id: Uuid,
    pub item_slug: String,
    pub item_data: &'static ItemData,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct RuntimeStatBlock {
    pub base: CombatStats,
    pub from_class: CombatStats,
    pub from_equipment: CombatStats,
    pub from_persistent_modifiers: CombatStats,
    pub from_timed_modifiers: CombatStats,
    pub final_stats: CombatStats,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct RuntimeRewardBlock {
    pub base: RewardStats,
    pub from_class: RewardStats,
    pub from_equipment: RewardStats,
    pub from_persistent_modifiers: RewardStats,
    pub from_timed_modifiers: RewardStats,
    pub final_stats: RewardStats,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct RuntimeResources {
    pub current_hp: i32,
    pub current_mp: i32,
}

impl RuntimeCharacter {
    pub fn recalculate_stats(&mut self) {
        let (persistent_stats, persistent_rewards) = aggregate_modifier_stats(
            &self.stats.base,
            &self.stats.from_class,
            &self.stats.from_equipment,
            &self.rewards.base,
            &self.rewards.from_class,
            &self.rewards.from_equipment,
            &self.persistent_modifiers,
        );
        let (timed_stats, timed_rewards) = aggregate_modifier_stats(
            &self.stats.base,
            &self.stats.from_class,
            &self.stats.from_equipment,
            &self.rewards.base,
            &self.rewards.from_class,
            &self.rewards.from_equipment,
            &self.timed_modifiers,
        );

        let mut final_stats = self.stats.base;
        final_stats += self.stats.from_class;
        final_stats += self.stats.from_equipment;
        final_stats += persistent_stats;
        final_stats += timed_stats;

        let mut final_rewards = self.rewards.base;
        final_rewards += self.rewards.from_class;
        final_rewards += self.rewards.from_equipment;
        final_rewards += persistent_rewards;
        final_rewards += timed_rewards;

        self.stats.from_persistent_modifiers = persistent_stats;
        self.stats.from_timed_modifiers = timed_stats;
        self.stats.final_stats = final_stats;
        self.rewards.from_persistent_modifiers = persistent_rewards;
        self.rewards.from_timed_modifiers = timed_rewards;
        self.rewards.final_stats = final_rewards;
        self.resources.current_hp = self
            .resources
            .current_hp
            .clamp(0, self.stats.final_stats.core.hp);
        self.resources.current_mp = self
            .resources
            .current_mp
            .clamp(0, self.stats.final_stats.core.mp);
    }

    pub fn add_persistent_modifier(&mut self, modifier: RuntimeModifier) {
        self.persistent_modifiers.push(modifier);
        self.recalculate_stats();
    }

    pub fn add_timed_modifier(&mut self, modifier: RuntimeModifier) {
        self.timed_modifiers.push(modifier);
        self.recalculate_stats();
    }

    pub fn tick_timed_modifiers(&mut self, elapsed_ms: u64) -> bool {
        let initial_len = self.timed_modifiers.len();
        self.timed_modifiers
            .retain_mut(|modifier| !modifier.tick(elapsed_ms));
        let changed = self.timed_modifiers.len() != initial_len;
        if changed {
            self.recalculate_stats();
        }
        changed
    }

    pub fn tick_skill_cooldowns(&mut self, elapsed_ms: u64) -> bool {
        let initial_len = self.skill_cooldowns_ms.len();
        self.skill_cooldowns_ms.retain(|_, remaining_ms| {
            *remaining_ms = remaining_ms.saturating_sub(elapsed_ms);
            *remaining_ms > 0
        });

        self.skill_cooldowns_ms.len() != initial_len
    }

    pub fn spend_mp(&mut self, amount: i32) -> Result<(), String> {
        if amount < 0 {
            return Err("mp cost cannot be negative".to_string());
        }
        if self.resources.current_mp < amount {
            return Err("not enough MP".to_string());
        }

        self.resources.current_mp -= amount;
        Ok(())
    }

    pub fn is_skill_on_cooldown(&self, skill_slug: &str) -> bool {
        self.skill_cooldowns_ms
            .get(skill_slug)
            .copied()
            .unwrap_or(0)
            > 0
    }

    pub fn set_skill_cooldown(&mut self, skill_slug: &str, remaining_ms: u64) {
        if remaining_ms > 0 {
            self.skill_cooldowns_ms
                .insert(skill_slug.to_string(), remaining_ms);
        }
    }

    pub fn active_buffs(&self) -> Vec<shared::net::ActiveBuffState> {
        self.timed_modifiers
            .iter()
            .filter_map(|modifier| match (&modifier.source, modifier.duration) {
                (
                    crate::runtime::modifier::ModifierSource::ActiveBuff { effect_slug },
                    crate::runtime::modifier::ModifierDuration::Timed { remaining_ms },
                ) => Some(shared::net::ActiveBuffState {
                    effect_slug: effect_slug.clone(),
                    remaining_ms,
                }),
                _ => None,
            })
            .collect()
    }

    pub fn skill_cooldowns(&self) -> Vec<shared::net::SkillCooldownState> {
        self.skill_cooldowns_ms
            .iter()
            .map(
                |(skill_slug, remaining_ms)| shared::net::SkillCooldownState {
                    skill_slug: skill_slug.clone(),
                    remaining_ms: *remaining_ms,
                },
            )
            .collect()
    }
}

fn aggregate_modifier_stats(
    base_stats: &CombatStats,
    class_stats: &CombatStats,
    equipment_stats: &CombatStats,
    base_rewards: &RewardStats,
    class_rewards: &RewardStats,
    equipment_rewards: &RewardStats,
    modifiers: &[RuntimeModifier],
) -> (CombatStats, RewardStats) {
    let mut running_stats = *base_stats;
    running_stats += *class_stats;
    running_stats += *equipment_stats;
    let mut running_rewards = *base_rewards;
    running_rewards += *class_rewards;
    running_rewards += *equipment_rewards;

    let mut contributed = CombatStats::ZERO;
    let mut contributed_rewards = RewardStats::ZERO;

    for modifier in modifiers {
        let before_stats = running_stats;
        let before_rewards = running_rewards;
        modifier.apply_to_stats(&mut running_stats, &mut running_rewards);
        contributed += difference(&before_stats, &running_stats);
        contributed_rewards += reward_difference(&before_rewards, &running_rewards);
    }

    (contributed, contributed_rewards)
}

fn difference(before: &CombatStats, after: &CombatStats) -> CombatStats {
    CombatStats {
        core: shared::models::combat_stats::CombatCoreStats {
            hp: after.core.hp - before.core.hp,
            mp: after.core.mp - before.core.mp,
            physical_atk: after.core.physical_atk - before.core.physical_atk,
            magical_atk: after.core.magical_atk - before.core.magical_atk,
            physical_def: after.core.physical_def - before.core.physical_def,
            magical_def: after.core.magical_def - before.core.magical_def,
            move_spd: after.core.move_spd - before.core.move_spd,
            atk_spd: after.core.atk_spd - before.core.atk_spd,
        },
        secondary: shared::models::combat_stats::CombatSecondaryStats {
            damage_reduction: after.secondary.damage_reduction - before.secondary.damage_reduction,
            crit_chance: after.secondary.crit_chance - before.secondary.crit_chance,
            crit_damage: after.secondary.crit_damage - before.secondary.crit_damage,
            accuracy: after.secondary.accuracy - before.secondary.accuracy,
            physical_attack_level: after.secondary.physical_attack_level
                - before.secondary.physical_attack_level,
            magical_attack_level: after.secondary.magical_attack_level
                - before.secondary.magical_attack_level,
            physical_pen: after.secondary.physical_pen - before.secondary.physical_pen,
            magical_pen: after.secondary.magical_pen - before.secondary.magical_pen,
            hp_regen: after.secondary.hp_regen - before.secondary.hp_regen,
            mp_regen: after.secondary.mp_regen - before.secondary.mp_regen,
            life_steal: after.secondary.life_steal - before.secondary.life_steal,
            cooldown_reduction: after.secondary.cooldown_reduction
                - before.secondary.cooldown_reduction,
            crit_resistance: after.secondary.crit_resistance - before.secondary.crit_resistance,
            knockback_resistance: after.secondary.knockback_resistance
                - before.secondary.knockback_resistance,
            cc_resistance: after.secondary.cc_resistance - before.secondary.cc_resistance,
        },
    }
}

fn reward_difference(before: &RewardStats, after: &RewardStats) -> RewardStats {
    RewardStats {
        experience_gain: after.experience_gain - before.experience_gain,
        drop_rate: after.drop_rate - before.drop_rate,
        credit_gain: after.credit_gain - before.credit_gain,
    }
}
