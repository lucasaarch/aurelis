use crate::models::dungeon_data::DungeonData;

pub mod stage_groups;

pub static CLOCK_TOWER_SQUARE: DungeonData = DungeonData {
    slug: "clock_tower_square",
    name: "Clock Tower Square",
    level_min: 0,
    level_max: 0,
    stage_groups: &[&stage_groups::PLAZA, &stage_groups::WARDEN_SANCTUM],
};

pub static CATHEDRAL_OF_SOLENNE: DungeonData = DungeonData {
    slug: "cathedral_of_solenne",
    name: "Cathedral of Solenne",
    level_min: 0,
    level_max: 0,
    stage_groups: &[
        &stage_groups::CATHEDRAL_NAVE,
        &stage_groups::ALTAR_OF_SILENCE,
    ],
};

pub static THE_GILDED_SEWERS: DungeonData = DungeonData {
    slug: "the_gilded_sewers",
    name: "The Gilded Sewers",
    level_min: 0,
    level_max: 0,
    stage_groups: &[
        &stage_groups::UPPER_SEWERS,
        &stage_groups::LOWER_SEWERS,
        &stage_groups::PLAGUE_ALTAR,
    ],
};

pub static ROYAL_ARCHIVE_DEPTHS: DungeonData = DungeonData {
    slug: "royal_archive_depths",
    name: "Royal Archive Depths",
    level_min: 0,
    level_max: 0,
    stage_groups: &[
        &stage_groups::ARCHIVE_UPPER,
        &stage_groups::ARCHIVE_LOWER,
        &stage_groups::ARCHIVIST_CHAMBER,
    ],
};

pub static SUNGUARD_BARRACKS: DungeonData = DungeonData {
    slug: "sunguard_barracks",
    name: "Sunguard Barracks",
    level_min: 0,
    level_max: 0,
    stage_groups: &[
        &stage_groups::BARRACKS_GROUNDS,
        &stage_groups::BARRACKS_INTERIOR,
        &stage_groups::COMMANDER_HALL,
    ],
};

pub static THE_SUNKEN_THRONE: DungeonData = DungeonData {
    slug: "the_sunken_throne",
    name: "The Sunken Throne",
    level_min: 0,
    level_max: 0,
    stage_groups: &[
        &stage_groups::FLOODED_ANTECHAMBER,
        &stage_groups::ROYAL_DEPTHS,
        &stage_groups::FORSAKEN_PASSAGE,
        &stage_groups::THRONE_ROOM,
    ],
};

pub static DUNGEONS: &[&DungeonData] = &[
    &CLOCK_TOWER_SQUARE,
    &CATHEDRAL_OF_SOLENNE,
    &THE_GILDED_SEWERS,
    &ROYAL_ARCHIVE_DEPTHS,
    &SUNGUARD_BARRACKS,
    &THE_SUNKEN_THRONE,
];
