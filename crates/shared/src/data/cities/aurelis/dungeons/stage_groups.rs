use crate::models::dungeon_data::{DungeonMobEntry, SpawnPoint, Stage, StageGroup, StageGroupKind};

const EMPTY_MOBS: &[DungeonMobEntry] = &[];

pub static PLAZA: StageGroup = StageGroup {
    slug: "plaza",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }],
    },
};

pub static WARDEN_SANCTUM: StageGroup = StageGroup {
    slug: "warden_sanctum",
    kind: StageGroupKind::Boss {
        bosses: &[DungeonMobEntry {
            mob_slug: "malgrath_defiled_warden",
            spawn_point: SpawnPoint { x: 0.0, y: 0.0 },
        }],
    },
};

pub static CATHEDRAL_NAVE: StageGroup = StageGroup {
    slug: "cathedral_nave",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static ALTAR_OF_SILENCE: StageGroup = StageGroup {
    slug: "altar_of_silence",
    kind: StageGroupKind::Boss {
        bosses: &[DungeonMobEntry {
            mob_slug: "the_hollow_choir",
            spawn_point: SpawnPoint { x: 0.0, y: 0.0 },
        }],
    },
};

pub static UPPER_SEWERS: StageGroup = StageGroup {
    slug: "upper_sewers",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static LOWER_SEWERS: StageGroup = StageGroup {
    slug: "lower_sewers",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static PLAGUE_ALTAR: StageGroup = StageGroup {
    slug: "plague_altar",
    kind: StageGroupKind::Boss {
        bosses: &[DungeonMobEntry {
            mob_slug: "rotcleaver_plague_deacon",
            spawn_point: SpawnPoint { x: 0.0, y: 0.0 },
        }],
    },
};

pub static ARCHIVE_UPPER: StageGroup = StageGroup {
    slug: "archive_upper",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static ARCHIVE_LOWER: StageGroup = StageGroup {
    slug: "archive_lower",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static ARCHIVIST_CHAMBER: StageGroup = StageGroup {
    slug: "archivist_chamber",
    kind: StageGroupKind::Boss {
        bosses: &[DungeonMobEntry {
            mob_slug: "archivist_voss_sealed_mind",
            spawn_point: SpawnPoint { x: 0.0, y: 0.0 },
        }],
    },
};

pub static BARRACKS_GROUNDS: StageGroup = StageGroup {
    slug: "barracks_grounds",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static BARRACKS_INTERIOR: StageGroup = StageGroup {
    slug: "barracks_interior",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static COMMANDER_HALL: StageGroup = StageGroup {
    slug: "commander_hall",
    kind: StageGroupKind::Boss {
        bosses: &[DungeonMobEntry {
            mob_slug: "commander_aldren_oathbreaker",
            spawn_point: SpawnPoint { x: 0.0, y: 0.0 },
        }],
    },
};

pub static FLOODED_ANTECHAMBER: StageGroup = StageGroup {
    slug: "flooded_antechamber",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static ROYAL_DEPTHS: StageGroup = StageGroup {
    slug: "royal_depths",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static FORSAKEN_PASSAGE: StageGroup = StageGroup {
    slug: "forsaken_passage",
    kind: StageGroupKind::Normal {
        stages: &[Stage { mobs: EMPTY_MOBS }, Stage { mobs: EMPTY_MOBS }],
    },
};

pub static THRONE_ROOM: StageGroup = StageGroup {
    slug: "throne_room",
    kind: StageGroupKind::Boss {
        bosses: &[DungeonMobEntry {
            mob_slug: "aldric_forsaken_king",
            spawn_point: SpawnPoint { x: 0.0, y: 0.0 },
        }],
    },
};
