use crate::models::{
    character_data::CombatAffinity,
    skill_data::{
        CharacterSkillUnlockTier, SkillCost, SkillData, SkillKind, SkillOwner,
        SkillUnlockRequirement, SpecialActiveTier,
    },
};

pub static SENTINEL_STEEL_PULSE: SkillData = SkillData {
    slug: "sentinel_steel_pulse",
    name: "Steel Pulse",
    description: "A disciplined pulse of force that fortifies Kael and disrupts nearby enemies.",
    owner: SkillOwner::Class {
        class_slug: "kael_royal_sentinel",
    },
    kind: SkillKind::Advantage,
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::Level { required_level: 10 },
    cost: SkillCost::Mp(36),
    cooldown_secs: 12.0,
    cast_time_secs: 0.6,
    range: 3.5,
};

pub static SENTINEL_SIGNATURE_DECREE: SkillData = SkillData {
    slug: "sentinel_signature_decree",
    name: "Signature Decree",
    description: "Kael invokes a royal command that surges through his blade and empowers his stance.",
    owner: SkillOwner::Class {
        class_slug: "kael_royal_sentinel",
    },
    kind: SkillKind::SpecialActive(SpecialActiveTier::Signature),
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::CharacterBookTier {
        required_level: 15,
        tier: CharacterSkillUnlockTier::Beginner,
    },
    cost: SkillCost::Mp(100),
    cooldown_secs: 28.0,
    cast_time_secs: 0.8,
    range: 4.0,
};
