use crate::models::{
    combat_stats::{FixedStatLine, StatKey},
    equipment_slot::EquipmentSlot,
    inventory_type::InventoryType,
    item_data::{
        ArmorData, CatalogStatModifier, CatalogStatModifierDefinition,
        EquipmentIdentificationRules, GemSlotConfig, ItemAcquisition, ItemData, ItemKind, Tradable,
        TradeFee, WeaponData,
    },
    item_instance_attributes::{EquipmentRollBias, StatModifierValueKind},
    item_rarity::ItemRarity,
    stat_modifier::ModifierStat,
};

const KAEL_TRAINING_BLADE_STATS: &[FixedStatLine] = &[FixedStatLine {
    stat: StatKey::PhysicalAtk,
    value: 42,
}];

const KAEL_SQUIRE_CHESTPLATE_STATS: &[FixedStatLine] = &[
    FixedStatLine {
        stat: StatKey::Hp,
        value: 120,
    },
    FixedStatLine {
        stat: StatKey::PhysicalDef,
        value: 18,
    },
    FixedStatLine {
        stat: StatKey::MagicalDef,
        value: 10,
    },
];

const KAEL_SQUIRE_LEGGUARDS_STATS: &[FixedStatLine] = &[
    FixedStatLine {
        stat: StatKey::DamageReduction,
        value: 6,
    },
    FixedStatLine {
        stat: StatKey::PhysicalDef,
        value: 16,
    },
    FixedStatLine {
        stat: StatKey::MagicalDef,
        value: 8,
    },
];

const KAEL_SQUIRE_GAUNTLETS_STATS: &[FixedStatLine] = &[
    FixedStatLine {
        stat: StatKey::AtkSpd,
        value: 7,
    },
    FixedStatLine {
        stat: StatKey::PhysicalDef,
        value: 12,
    },
    FixedStatLine {
        stat: StatKey::MagicalDef,
        value: 6,
    },
];

const KAEL_SQUIRE_BOOTS_STATS: &[FixedStatLine] = &[
    FixedStatLine {
        stat: StatKey::MoveSpd,
        value: 9,
    },
    FixedStatLine {
        stat: StatKey::PhysicalDef,
        value: 10,
    },
    FixedStatLine {
        stat: StatKey::MagicalDef,
        value: 5,
    },
];

const KAEL_TRAINING_BLADE_FIXED_EFFECTS: &[CatalogStatModifier] = &[
    CatalogStatModifier {
        id: "weapon_base_crit_chance",
        stat: ModifierStat::Combat(StatKey::CritChance),
        kind: StatModifierValueKind::Flat,
        value: 10,
    },
    CatalogStatModifier {
        id: "weapon_base_crit_damage",
        stat: ModifierStat::Combat(StatKey::CritDamage),
        kind: StatModifierValueKind::Flat,
        value: 5,
    },
];

const KAEL_TRAINING_BLADE_ADDITIONAL_EFFECT_POOL: &[CatalogStatModifierDefinition] = &[
    CatalogStatModifierDefinition {
        id: "add_physical_attack_level",
        stat: ModifierStat::Combat(StatKey::PhysicalAttackLevel),
        kind: StatModifierValueKind::Flat,
        min_value: 300,
        max_value: 450,
        weight: 100,
    },
    CatalogStatModifierDefinition {
        id: "add_magical_attack_level",
        stat: ModifierStat::Combat(StatKey::MagicalAttackLevel),
        kind: StatModifierValueKind::Flat,
        min_value: 300,
        max_value: 450,
        weight: 100,
    },
    CatalogStatModifierDefinition {
        id: "add_crit_damage",
        stat: ModifierStat::Combat(StatKey::CritDamage),
        kind: StatModifierValueKind::Flat,
        min_value: 3,
        max_value: 8,
        weight: 75,
    },
];

const NO_FIXED_SPECIAL_EFFECTS: &[CatalogStatModifier] = &[];
const NO_ADDITIONAL_EFFECT_POOL: &[CatalogStatModifierDefinition] = &[];

pub static KAEL_TRAINING_BLADE: ItemData = ItemData {
    slug: "kael_training_blade",
    name: "Kael Training Blade",
    description: "A balanced practice sword issued to new wardens of Aurelis.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Equipment,
    max_stack: 1,
    kind: ItemKind::Weapon(WeaponData {
        slot: EquipmentSlot::Weapon,
        class: Some("kael"),
        level_req: 1,
        fixed_stats: KAEL_TRAINING_BLADE_STATS,
        fixed_special_effects: KAEL_TRAINING_BLADE_FIXED_EFFECTS,
        gem_slots: GemSlotConfig::FOUR_BASE_PLUS_ONE_BONUS,
        identification: Some(EquipmentIdentificationRules {
            starts_unidentified: true,
            bias: EquipmentRollBias::Physical,
            additional_effect_count: 3,
            additional_effect_pool: KAEL_TRAINING_BLADE_ADDITIONAL_EFFECT_POOL,
        }),
    }),
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: TradeFee::Free,
        },
    },
};

pub static KAEL_SQUIRE_CHESTPLATE: ItemData = ItemData {
    slug: "kael_squire_chestplate",
    name: "Kael Squire Chestplate",
    description: "A sturdy chestplate designed to keep young wardens alive a little longer.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Equipment,
    max_stack: 1,
    kind: ItemKind::Armor(ArmorData {
        slot: EquipmentSlot::Chest,
        level_req: 1,
        fixed_stats: KAEL_SQUIRE_CHESTPLATE_STATS,
        fixed_special_effects: NO_FIXED_SPECIAL_EFFECTS,
        gem_slots: GemSlotConfig::FOUR_BASE_PLUS_ONE_BONUS,
        identification: Some(EquipmentIdentificationRules {
            starts_unidentified: true,
            bias: EquipmentRollBias::Neutral,
            additional_effect_count: 0,
            additional_effect_pool: NO_ADDITIONAL_EFFECT_POOL,
        }),
    }),
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: TradeFee::Free,
        },
    },
};

pub static KAEL_SQUIRE_LEGGUARDS: ItemData = ItemData {
    slug: "kael_squire_legguards",
    name: "Kael Squire Legguards",
    description: "Weighted legguards that trade speed for steadiness in combat.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Equipment,
    max_stack: 1,
    kind: ItemKind::Armor(ArmorData {
        slot: EquipmentSlot::Legs,
        level_req: 1,
        fixed_stats: KAEL_SQUIRE_LEGGUARDS_STATS,
        fixed_special_effects: NO_FIXED_SPECIAL_EFFECTS,
        gem_slots: GemSlotConfig::FOUR_BASE_PLUS_ONE_BONUS,
        identification: Some(EquipmentIdentificationRules {
            starts_unidentified: true,
            bias: EquipmentRollBias::Neutral,
            additional_effect_count: 0,
            additional_effect_pool: NO_ADDITIONAL_EFFECT_POOL,
        }),
    }),
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: TradeFee::Free,
        },
    },
};

pub static KAEL_SQUIRE_GAUNTLETS: ItemData = ItemData {
    slug: "kael_squire_gauntlets",
    name: "Kael Squire Gauntlets",
    description: "Flexible gauntlets that improve grip and striking cadence.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Equipment,
    max_stack: 1,
    kind: ItemKind::Armor(ArmorData {
        slot: EquipmentSlot::Gloves,
        level_req: 1,
        fixed_stats: KAEL_SQUIRE_GAUNTLETS_STATS,
        fixed_special_effects: NO_FIXED_SPECIAL_EFFECTS,
        gem_slots: GemSlotConfig::FOUR_BASE_PLUS_ONE_BONUS,
        identification: Some(EquipmentIdentificationRules {
            starts_unidentified: true,
            bias: EquipmentRollBias::Neutral,
            additional_effect_count: 0,
            additional_effect_pool: NO_ADDITIONAL_EFFECT_POOL,
        }),
    }),
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: TradeFee::Free,
        },
    },
};

pub static KAEL_SQUIRE_BOOTS: ItemData = ItemData {
    slug: "kael_squire_boots",
    name: "Kael Squire Boots",
    description: "Boots reinforced for long marches and quick battlefield repositioning.",
    rarity: ItemRarity::Common,
    inventory_type: InventoryType::Equipment,
    max_stack: 1,
    kind: ItemKind::Armor(ArmorData {
        slot: EquipmentSlot::Shoes,
        level_req: 1,
        fixed_stats: KAEL_SQUIRE_BOOTS_STATS,
        fixed_special_effects: NO_FIXED_SPECIAL_EFFECTS,
        gem_slots: GemSlotConfig::FOUR_BASE_PLUS_ONE_BONUS,
        identification: Some(EquipmentIdentificationRules {
            starts_unidentified: true,
            bias: EquipmentRollBias::Neutral,
            additional_effect_count: 0,
            additional_effect_pool: NO_ADDITIONAL_EFFECT_POOL,
        }),
    }),
    acquisition: ItemAcquisition {
        droppable: true,
        purchasable: None,
        sellable: None,
        tradable: Tradable::Yes {
            fee: TradeFee::Free,
        },
    },
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
    &KAEL_TRAINING_BLADE,
    &KAEL_SQUIRE_CHESTPLATE,
    &KAEL_SQUIRE_LEGGUARDS,
    &KAEL_SQUIRE_GAUNTLETS,
    &KAEL_SQUIRE_BOOTS,
    &CRACKED_STONE_FRAGMENT,
    &DARK_FEATHER,
    &CORRUPTED_HOLY_WATER,
    &VOID_SHARD,
    &WARDENS_SEAL,
];
