use crate::models::character_data::{CharacterBaseStats, ClassData, ClassType};

pub static KAEL_ROYAL_SENTINEL: ClassData = ClassData {
    slug: "kael_royal_sentinel",
    name: "Royal Sentinel",
    description: "Kael embraces his duty as a protector of Aurelis, hardening his body and sharpening his blade. His strikes become more deliberate, trading speed for devastating precision.",
    class_type: ClassType::First,
    stat_bonuses: CharacterBaseStats {
        hp: 320,
        mp: 100,
        physical_atk: 32,
        magical_atk: 4,
        physical_def: 22,
        magical_def: 8,
        move_spd: 5,
        atk_spd: 8,
    },
};

pub static KAEL_SOVEREIGN_BLADE: ClassData = ClassData {
    slug: "kael_sovereign_blade",
    name: "Sovereign Blade",
    description: "Having witnessed the full weight of Aurelis's fall, Kael transcends his role as a soldier. He becomes a force of reckoning — his blade now carries the condensed will of a kingdom.",
    class_type: ClassType::Second,
    stat_bonuses: CharacterBaseStats {
        hp: 680,
        mp: 220,
        physical_atk: 85,
        magical_atk: 10,
        physical_def: 55,
        magical_def: 18,
        move_spd: 10,
        atk_spd: 12,
    },
};
