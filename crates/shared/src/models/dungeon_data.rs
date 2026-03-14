pub struct DungeonData {
    pub slug: &'static str,
    pub name: &'static str,
    pub level_min: i16,
    pub level_max: i16,
    pub stage_groups: &'static [&'static StageGroup],
}

pub struct StageGroup {
    pub slug: &'static str,
    pub kind: StageGroupKind,
}

pub enum StageGroupKind {
    Normal { stages: &'static [Stage] },
    Boss { bosses: &'static [DungeonMobEntry] },
}

pub struct Stage {
    pub mobs: &'static [DungeonMobEntry],
}

pub struct DungeonMobEntry {
    pub mob_slug: &'static str,
    pub spawn_point: SpawnPoint,
}

pub struct SpawnPoint {
    pub x: f32,
    pub y: f32,
}
