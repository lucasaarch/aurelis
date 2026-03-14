use crate::models::quest_data::{
    QuestActivation, QuestCadence, QuestCategory, QuestData, QuestDungeonTarget, QuestItemReward,
    QuestMobTarget, QuestObjective, QuestRewards, QuestScope, QuestTrigger,
};

pub static AURELIS_FIRST_STEPS: QuestData = QuestData {
    slug: "aurelis_first_steps",
    name: "First Steps Into the Square",
    description: "Enter Clock Tower Square and clear out the first corrupted creatures threatening the road into Aurelis.",
    scope: QuestScope::Character,
    category: QuestCategory::Story,
    cadence: QuestCadence::Normal,
    level_req: 1,
    activation: QuestActivation::Manual,
    objective: QuestObjective::KillMobs {
        targets: &[
            QuestMobTarget {
                mob_slug: "tarnished_sentinel",
                quantity: 6,
            },
            QuestMobTarget {
                mob_slug: "ashwing_crow",
                quantity: 4,
            },
        ],
    },
    rewards: QuestRewards {
        experience: 180,
        credits: 120,
        guaranteed_items: &[
            QuestItemReward {
                item_slug: "cracked_stone_fragment",
                quantity: 12,
            },
            QuestItemReward {
                item_slug: "dark_feather",
                quantity: 8,
            },
        ],
        selectable_items: &[],
    },
};

pub static WARDEN_OF_THE_SQUARE: QuestData = QuestData {
    slug: "warden_of_the_square",
    name: "Warden of the Square",
    description: "Defeat Malgrath, the Defiled Warden, and break the corruption holding Clock Tower Square.",
    scope: QuestScope::Account,
    category: QuestCategory::Story,
    cadence: QuestCadence::Normal,
    level_req: 1,
    activation: QuestActivation::Automatic(QuestTrigger::QuestCompleted {
        quest_slug: "aurelis_first_steps",
    }),
    objective: QuestObjective::ClearDungeons {
        targets: &[QuestDungeonTarget {
            dungeon_slug: "clock_tower_square",
            quantity: 1,
        }],
    },
    rewards: QuestRewards {
        experience: 300,
        credits: 220,
        guaranteed_items: &[QuestItemReward {
            item_slug: "wardens_seal",
            quantity: 10,
        }],
        selectable_items: &[
            QuestItemReward {
                item_slug: "cracked_stone_fragment",
                quantity: 30,
            },
            QuestItemReward {
                item_slug: "dark_feather",
                quantity: 30,
            },
            QuestItemReward {
                item_slug: "void_shard",
                quantity: 20,
            },
        ],
    },
};

pub static QUESTS: &[&QuestData] = &[&AURELIS_FIRST_STEPS, &WARDEN_OF_THE_SQUARE];
