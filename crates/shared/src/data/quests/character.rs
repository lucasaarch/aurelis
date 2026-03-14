use crate::models::quest_data::{
    QuestActivation, QuestCadence, QuestCategory, QuestData, QuestItemReward, QuestItemTarget,
    QuestObjective, QuestRewards, QuestScope,
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

pub static QUESTS: &[&QuestData] = &[&SIGNS_OF_CORRUPTION];
