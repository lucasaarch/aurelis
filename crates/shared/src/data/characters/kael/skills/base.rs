use crate::models::{
    character_data::CombatAffinity,
    item_data::CatalogStatModifier,
    skill_data::{SkillCost, SkillData, SkillKind, SkillOwner, SkillUnlockRequirement},
};

const NO_SKILL_MODIFIERS: &[CatalogStatModifier] = &[];

pub static KAEL_SLASH: SkillData = SkillData {
    slug: "kael_slash",
    name: "Royal Slash",
    description: "A precise forward slash that deals solid physical damage.",
    owner: SkillOwner::BaseCharacter {
        character_slug: "kael",
    },
    kind: SkillKind::Active,
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::Level { required_level: 1 },
    cost: SkillCost::Mp(12),
    cooldown_secs: 4.0,
    cast_time_secs: 0.2,
    range: 1.8,
    passive_modifiers: NO_SKILL_MODIFIERS,
    timed_buff: None,
};

pub static KAEL_GUARDING_STRIKE: SkillData = SkillData {
    slug: "kael_guarding_strike",
    name: "Guarding Strike",
    description: "A guarded thrust that damages the target and steadies Kael's stance.",
    owner: SkillOwner::BaseCharacter {
        character_slug: "kael",
    },
    kind: SkillKind::Advantage,
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::Level { required_level: 5 },
    cost: SkillCost::Mp(18),
    cooldown_secs: 8.0,
    cast_time_secs: 0.4,
    range: 2.2,
    passive_modifiers: NO_SKILL_MODIFIERS,
    timed_buff: None,
};
