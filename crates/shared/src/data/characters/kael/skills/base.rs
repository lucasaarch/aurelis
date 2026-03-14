use crate::models::skill_data::{SkillData, SkillKind, SkillOwner};

pub static KAEL_SLASH: SkillData = SkillData {
    slug: "kael_slash",
    name: "Royal Slash",
    description: "A precise forward slash that deals solid physical damage.",
    owner: SkillOwner::BaseCharacter {
        character_slug: "kael",
    },
    kind: SkillKind::SpecialActive,
    level_req: 1,
    mp_cost: 12,
    cooldown_secs: 4.0,
    cast_time_secs: 0.2,
    range: 1.8,
};

pub static KAEL_GUARDING_STRIKE: SkillData = SkillData {
    slug: "kael_guarding_strike",
    name: "Guarding Strike",
    description: "A guarded thrust that damages the target and steadies Kael's stance.",
    owner: SkillOwner::BaseCharacter {
        character_slug: "kael",
    },
    kind: SkillKind::SpecialActive,
    level_req: 1,
    mp_cost: 18,
    cooldown_secs: 8.0,
    cast_time_secs: 0.4,
    range: 2.2,
};
