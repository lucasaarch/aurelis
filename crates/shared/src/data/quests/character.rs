use crate::models::quest_data::{
    QuestActivation, QuestCadence, QuestCategory, QuestData, QuestItemReward, QuestItemTarget,
    QuestObjective, QuestRewards, QuestScope, QuestTrigger,
};

pub static SIGNS_OF_CORRUPTION: QuestData = QuestData {
    slug: "signs_of_corruption",
    name: "Signs of Corruption",
    description: "Recover traces of corruption from the square so the scouts can understand what is spreading through Aurelis.",
    scope: QuestScope::Character,
    category: QuestCategory::Character,
    cadence: QuestCadence::Normal,
    level_req: 1,
    activation: QuestActivation::Manual,
    objective: QuestObjective::CollectItems {
        targets: &[
            QuestItemTarget {
                item_slug: "corrupted_holy_water",
                quantity: 5,
            },
            QuestItemTarget {
                item_slug: "void_shard",
                quantity: 3,
            },
        ],
    },
    rewards: QuestRewards {
        experience: 140,
        credits: 90,
        guaranteed_items: &[QuestItemReward {
            item_slug: "wardens_seal",
            quantity: 3,
        }],
        selectable_items: &[],
    },
};

pub static BEGINNER_LEARNING_BOOK_TRIAL: QuestData = QuestData {
    slug: "beginner_learning_book_trial",
    name: "First Tome of Evolution",
    description: "Prove your growth is worthy of a new teaching and receive the beginner learning book.",
    scope: QuestScope::Character,
    category: QuestCategory::Character,
    cadence: QuestCadence::Normal,
    level_req: 15,
    activation: QuestActivation::Automatic(QuestTrigger::CharacterLevelReached { level: 15 }),
    objective: QuestObjective::TalkToNpc {
        npc_slug: "training_master_kael",
    },
    rewards: QuestRewards {
        experience: 250,
        credits: 180,
        guaranteed_items: &[QuestItemReward {
            item_slug: "learning_book_beginner",
            quantity: 1,
        }],
        selectable_items: &[],
    },
};

pub static INTERMEDIATE_LEARNING_BOOK_TRIAL: QuestData = QuestData {
    slug: "intermediate_learning_book_trial",
    name: "Second Tome of Evolution",
    description: "Return to the master of your lineage and receive the intermediate learning book for your next teaching.",
    scope: QuestScope::Character,
    category: QuestCategory::Character,
    cadence: QuestCadence::Normal,
    level_req: 35,
    activation: QuestActivation::Automatic(QuestTrigger::CharacterLevelReached { level: 35 }),
    objective: QuestObjective::TalkToNpc {
        npc_slug: "training_master_kael",
    },
    rewards: QuestRewards {
        experience: 420,
        credits: 320,
        guaranteed_items: &[QuestItemReward {
            item_slug: "learning_book_intermediate",
            quantity: 1,
        }],
        selectable_items: &[],
    },
};

pub static QUESTS: &[&QuestData] = &[
    &SIGNS_OF_CORRUPTION,
    &BEGINNER_LEARNING_BOOK_TRIAL,
    &INTERMEDIATE_LEARNING_BOOK_TRIAL,
];
