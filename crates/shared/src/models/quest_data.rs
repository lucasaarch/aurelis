pub struct QuestData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub scope: QuestScope,
    pub category: QuestCategory,
    pub cadence: QuestCadence,
    pub level_req: i16,
    pub activation: QuestActivation,
    pub objective: QuestObjective,
    pub rewards: QuestRewards,
}

pub enum QuestScope {
    Character,
    Account,
}

pub enum QuestCategory {
    Character,
    Story,
    Event,
}

pub enum QuestCadence {
    Normal,
    Daily,
    Weekly,
}

pub enum QuestActivation {
    Manual,
    Automatic(QuestTrigger),
}

pub enum QuestTrigger {
    FirstTravelToCity { city_slug: &'static str },
    CharacterLevelReached { level: i16 },
    QuestCompleted { quest_slug: &'static str },
    DungeonCleared { dungeon_slug: &'static str },
}

pub enum QuestObjective {
    KillMobs {
        targets: &'static [QuestMobTarget],
    },
    ClearDungeons {
        targets: &'static [QuestDungeonTarget],
    },
    CollectItems {
        targets: &'static [QuestItemTarget],
    },
    TalkToNpc {
        npc_slug: &'static str,
    },
}

pub struct QuestMobTarget {
    pub mob_slug: &'static str,
    pub quantity: i32,
}

pub struct QuestDungeonTarget {
    pub dungeon_slug: &'static str,
    pub quantity: i32,
}

pub struct QuestItemTarget {
    pub item_slug: &'static str,
    pub quantity: i32,
}

pub struct QuestRewards {
    pub experience: i32,
    pub credits: i32,
    pub guaranteed_items: &'static [QuestItemReward],
    pub selectable_items: &'static [QuestItemReward],
}

pub struct QuestItemReward {
    pub item_slug: &'static str,
    pub quantity: i32,
}
