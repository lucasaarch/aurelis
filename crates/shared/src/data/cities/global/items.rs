use crate::models::{
    inventory_type::InventoryType,
    item_data::{
        ChestContents, ChestData, ChestFixedEntry, ItemAcquisition, ItemData, ItemKind, ItemPrice,
        SpecialData, SpecialEffect, Tradable,
    },
    item_rarity::ItemRarity,
    skill_data::CharacterSkillUnlockTier,
};

pub static INVENTORY_EXPANSION_EQUIPMENT: ItemData = ItemData {
    slug: "inventory_expansion_equipment",
    name: "Inventory Expansion: Equipment",
    description: "Adds 1 row to the equipment inventory.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::InventoryExpansion {
            inventory_type: InventoryType::Equipment,
            rows: 1,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(500)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static INVENTORY_EXPANSION_ACCESSORY: ItemData = ItemData {
    slug: "inventory_expansion_accessory",
    name: "Inventory Expansion: Accessory",
    description: "Adds 1 row to the accessory inventory.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::InventoryExpansion {
            inventory_type: InventoryType::Accessory,
            rows: 1,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(500)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static INVENTORY_EXPANSION_CONSUMABLE: ItemData = ItemData {
    slug: "inventory_expansion_consumable",
    name: "Inventory Expansion: Consumable",
    description: "Adds 1 row to the consumable inventory.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::InventoryExpansion {
            inventory_type: InventoryType::Consumable,
            rows: 1,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(500)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static INVENTORY_EXPANSION_MATERIAL: ItemData = ItemData {
    slug: "inventory_expansion_material",
    name: "Inventory Expansion: Material",
    description: "Adds 1 row to the material inventory.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::InventoryExpansion {
            inventory_type: InventoryType::Material,
            rows: 1,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(500)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static INVENTORY_EXPANSION_QUEST_ITEM: ItemData = ItemData {
    slug: "inventory_expansion_quest_item",
    name: "Inventory Expansion: Quest Items",
    description: "Adds 1 row to the quest item inventory.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::InventoryExpansion {
            inventory_type: InventoryType::QuestItem,
            rows: 1,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(500)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static INVENTORY_EXPANSION_SPECIAL: ItemData = ItemData {
    slug: "inventory_expansion_special",
    name: "Inventory Expansion: Special",
    description: "Adds 1 row to the special inventory.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::InventoryExpansion {
            inventory_type: InventoryType::Special,
            rows: 1,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(500)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static INVENTORY_EXPANSION_KIT: ItemData = ItemData {
    slug: "inventory_expansion_kit",
    name: "Inventory Expansion Kit",
    description: "A chest containing one expansion for each inventory type.",
    rarity: ItemRarity::Uncommon,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::Chest(ChestData {
            contents: ChestContents::Fixed(&[
                ChestFixedEntry {
                    item_slug: INVENTORY_EXPANSION_EQUIPMENT.slug,
                    quantity: 1,
                },
                ChestFixedEntry {
                    item_slug: INVENTORY_EXPANSION_ACCESSORY.slug,
                    quantity: 1,
                },
                ChestFixedEntry {
                    item_slug: INVENTORY_EXPANSION_CONSUMABLE.slug,
                    quantity: 1,
                },
                ChestFixedEntry {
                    item_slug: INVENTORY_EXPANSION_MATERIAL.slug,
                    quantity: 1,
                },
                ChestFixedEntry {
                    item_slug: INVENTORY_EXPANSION_QUEST_ITEM.slug,
                    quantity: 1,
                },
                ChestFixedEntry {
                    item_slug: INVENTORY_EXPANSION_SPECIAL.slug,
                    quantity: 1,
                },
            ]),
        }),
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: Some(ItemPrice::Cash(2400)),
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static LEARNING_BOOK_BEGINNER: ItemData = ItemData {
    slug: "learning_book_beginner",
    name: "Beginner Learning Book",
    description: "Unlocks the level 15 locked skill for any evolution line of the character.",
    rarity: ItemRarity::Rare,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::CharacterSkillUnlock {
            tier: CharacterSkillUnlockTier::Beginner,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: None,
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static LEARNING_BOOK_INTERMEDIATE: ItemData = ItemData {
    slug: "learning_book_intermediate",
    name: "Intermediate Learning Book",
    description: "Unlocks the level 35 locked skill for any evolution line of the character.",
    rarity: ItemRarity::Epic,
    inventory_type: InventoryType::Special,
    max_stack: 1,
    kind: ItemKind::Special(SpecialData {
        effect: SpecialEffect::CharacterSkillUnlock {
            tier: CharacterSkillUnlockTier::Intermediate,
        },
    }),
    acquisition: ItemAcquisition {
        droppable: false,
        purchasable: None,
        sellable: None,
        tradable: Tradable::No,
    },
};

pub static ITEMS: &[&ItemData] = &[
    &INVENTORY_EXPANSION_EQUIPMENT,
    &INVENTORY_EXPANSION_ACCESSORY,
    &INVENTORY_EXPANSION_CONSUMABLE,
    &INVENTORY_EXPANSION_MATERIAL,
    &INVENTORY_EXPANSION_QUEST_ITEM,
    &INVENTORY_EXPANSION_SPECIAL,
    &INVENTORY_EXPANSION_KIT,
    &LEARNING_BOOK_BEGINNER,
    &LEARNING_BOOK_INTERMEDIATE,
];
