use crate::models::{
    character_data::CombatAffinity,
    skill_data::{
        CharacterSkillUnlockTier, SkillCost, SkillData, SkillKind, SkillOwner,
        SkillUnlockRequirement, SpecialActiveTier,
    },
};

pub static SOVEREIGN_BREAKER: SkillData = SkillData {
    slug: "sovereign_breaker",
    name: "Sovereign Breaker",
    description: "A crushing blade technique that tears through enemy formations with overwhelming force.",
    owner: SkillOwner::Class {
        class_slug: "kael_sovereign_blade",
    },
    kind: SkillKind::Active,
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::Level { required_level: 25 },
    cost: SkillCost::Mp(48),
    cooldown_secs: 16.0,
    cast_time_secs: 0.7,
    range: 2.8,
};

pub static ASCENDANT_KINGSFALL: SkillData = SkillData {
    slug: "ascendant_kingsfall",
    name: "Ascendant Kingsfall",
    description: "Kael channels the will of a fallen kingdom into a devastating awakened blade art.",
    owner: SkillOwner::Class {
        class_slug: "kael_sovereign_blade",
    },
    kind: SkillKind::SpecialActive(SpecialActiveTier::Awakened),
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::CharacterBookTier {
        required_level: 35,
        tier: CharacterSkillUnlockTier::Intermediate,
    },
    cost: SkillCost::Mp(200),
    cooldown_secs: 42.0,
    cast_time_secs: 1.1,
    range: 5.0,
};
