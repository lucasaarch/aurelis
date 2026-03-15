use crate::models::character_data::{CharacterBaseStats, CharacterData};

use self::paths::KAEL_ROYAL_BLADE;

pub mod classes;
pub mod paths;
pub mod skills;

pub static KAEL: CharacterData = CharacterData {
    slug: "kael",
    name: "Kael",
    description: "Son of the King of Aurelis, Kael wields his sword with precision and unwavering resolve. Once a proud guardian of the city's light, he now fights to reclaim what was taken from his people.",
    base_stats: CharacterBaseStats {
        hp: 30_000,
        mp: 300,
        physical_atk: 446,
        magical_atk: 446,
        physical_def: 113,
        magical_def: 113,
        move_spd: 100,
        atk_spd: 100,
        damage_reduction: 0,
        crit_chance: 0,
        crit_damage: 150,
        accuracy: 0,
        physical_attack_level: 0,
        magical_attack_level: 0,
        physical_pen: 0,
        magical_pen: 0,
        hp_regen: 0,
        mp_regen: 0,
        life_steal: 0,
        cooldown_reduction: 0,
        crit_resistance: 0,
        knockback_resistance: 0,
        cc_resistance: 0,
    },
    evolution_lines: &[&KAEL_ROYAL_BLADE],
};
