use crate::models::character_data::{CharacterData, CharacterSkillUnlocks, CombatAffinity};
use crate::models::item_data::CatalogStatModifier;

pub struct SkillData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub owner: SkillOwner,
    pub kind: SkillKind,
    pub scaling_affinity: CombatAffinity,
    pub unlock_requirement: SkillUnlockRequirement,
    pub cost: SkillCost,
    pub cooldown_secs: f32,
    pub cast_time_secs: f32,
    pub range: f32,
    pub passive_modifiers: &'static [CatalogStatModifier],
    pub timed_buff: Option<SkillTimedBuffData>,
}

pub enum SkillOwner {
    BaseCharacter { character_slug: &'static str },
    Class { class_slug: &'static str },
}

pub enum SkillKind {
    Active,
    Advantage,
    SpecialActive(SpecialActiveTier),
    Passive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialActiveTier {
    Signature,
    Awakened,
    Ascendant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSkillUnlockTier {
    Beginner,
    Intermediate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillUnlockRequirement {
    Level {
        required_level: i16,
    },
    CharacterBookTier {
        required_level: i16,
        tier: CharacterSkillUnlockTier,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillCost {
    None,
    Mp(i32),
}

pub struct SkillTimedBuffData {
    pub duration_ms: u64,
    pub modifiers: &'static [CatalogStatModifier],
}

impl SkillData {
    pub fn required_level(&self) -> i16 {
        match self.unlock_requirement {
            SkillUnlockRequirement::Level { required_level } => required_level,
            SkillUnlockRequirement::CharacterBookTier { required_level, .. } => required_level,
        }
    }

    pub fn mp_cost(&self) -> i32 {
        match self.cost {
            SkillCost::None => 0,
            SkillCost::Mp(value) => value,
        }
    }

    pub fn is_unlocked_for(
        &self,
        character: &CharacterData,
        current_class_slug: &str,
        character_level: i16,
        skill_unlocks: &CharacterSkillUnlocks,
    ) -> bool {
        if character_level < self.required_level() {
            return false;
        }

        let Some(unlocked_slugs) =
            character.unlocked_class_slugs_for_current_class(current_class_slug)
        else {
            return false;
        };

        let owner_unlocked = match self.owner {
            SkillOwner::BaseCharacter { character_slug } => {
                unlocked_slugs.contains(&character_slug)
            }
            SkillOwner::Class { class_slug } => unlocked_slugs.contains(&class_slug),
        };

        if !owner_unlocked {
            return false;
        }

        match self.unlock_requirement {
            SkillUnlockRequirement::Level { .. } => true,
            SkillUnlockRequirement::CharacterBookTier { tier, .. } => skill_unlocks.has_tier(tier),
        }
    }
}
