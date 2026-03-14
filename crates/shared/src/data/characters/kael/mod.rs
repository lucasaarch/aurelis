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
        hp: 520,
        mp: 180,
        physical_atk: 48,
        magical_atk: 8,
        physical_def: 32,
        magical_def: 14,
        move_spd: 100,
        atk_spd: 100,
    },
    evolution_lines: &[&KAEL_ROYAL_BLADE],
};
