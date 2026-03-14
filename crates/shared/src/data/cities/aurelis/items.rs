use crate::models::{
    inventory_type::InventoryType,
    item_data::{ItemAcquisition, ItemData, ItemKind, Tradable},
    item_rarity::ItemRarity,
};

pub static CRACKED_STONE_FRAGMENT: ItemData = ItemData {
    slug: "cracked_stone_fragment",
    name: "Cracked Stone Fragment",
    description: "A fractured shard of old stone recovered from ruined structures in Aurelis.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Material,
    max_stack: 500,
    kind: ItemKind::Material,
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: crate::models::item_data::TradeFee::Free,
        },
    },
};

pub static DARK_FEATHER: ItemData = ItemData {
    slug: "dark_feather",
    name: "Dark Feather",
    description: "A blackened feather tainted by the corruption spreading through Aurelis.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Material,
    max_stack: 500,
    kind: ItemKind::Material,
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: crate::models::item_data::TradeFee::Free,
        },
    },
};

pub static CORRUPTED_HOLY_WATER: ItemData = ItemData {
    slug: "corrupted_holy_water",
    name: "Corrupted Holy Water",
    description: "Once sacred water, now twisted by impurity and decay.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Material,
    max_stack: 500,
    kind: ItemKind::Material,
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: crate::models::item_data::TradeFee::Free,
        },
    },
};

pub static VOID_SHARD: ItemData = ItemData {
    slug: "void_shard",
    name: "Void Shard",
    description: "A shard resonating with unstable void energy.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Material,
    max_stack: 500,
    kind: ItemKind::Material,
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: crate::models::item_data::TradeFee::Free,
        },
    },
};

pub static WARDENS_SEAL: ItemData = ItemData {
    slug: "wardens_seal",
    name: "Warden's Seal",
    description: "A seal once carried by Aurelis wardens as proof of duty and rank.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Material,
    max_stack: 500,
    kind: ItemKind::Material,
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: crate::models::item_data::TradeFee::Free,
        },
    },
};

pub static ITEMS: &[&ItemData] = &[
    &CRACKED_STONE_FRAGMENT,
    &DARK_FEATHER,
    &CORRUPTED_HOLY_WATER,
    &VOID_SHARD,
    &WARDENS_SEAL,
];
