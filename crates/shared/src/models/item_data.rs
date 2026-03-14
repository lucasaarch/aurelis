use crate::models::{
    equipment_slot::EquipmentSlot, inventory_type::InventoryType, item_rarity::ItemRarity,
};

pub struct ItemData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: ItemRarity,
    pub inventory_type: InventoryType,
    pub max_stack: i32,
    pub kind: ItemKind,
    pub acquisition: ItemAcquisition,
}

pub struct ItemAcquisition {
    pub droppable: bool,
    pub purchasable: Option<ItemPrice>,
    pub sellable: Option<ItemPrice>,
    pub tradable: Tradable,
}

pub enum ItemPrice {
    Cash(i64),
    Credits(i64),
    Both { cash: i64, credits: i64 },
}

pub enum Tradable {
    No,
    Yes { fee: TradeFee },
}

pub enum TradeFee {
    Free,
    Credits(i64),
    Cash(i64),
}

pub enum ItemKind {
    Weapon(WeaponData),
    Armor(ArmorData),
    Material,
    Consumable(ConsumableData),
    Special(SpecialData),
}

pub struct WeaponData {
    pub slot: EquipmentSlot,
    pub class: Option<&'static str>,
    pub level_req: i16,
    pub stats: WeaponStats,
}

pub struct ArmorData {
    pub slot: EquipmentSlot,
    pub level_req: i16,
    pub stats: ArmorStats,
}

pub struct ConsumableData {
    pub duration_secs: Option<u32>,
    pub hp_restore_percent: Option<f32>,
    pub mp_restore_percent: Option<f32>,
    pub power_boost_percent: Option<f32>,
    pub move_speed_boost_percent: Option<f32>,
    pub attack_speed_boost_percent: Option<f32>,
}

pub struct WeaponStats {
    pub physical_atk: i32,
    pub magical_atk: i32,
}

pub struct ArmorStats {
    pub physical_def: i32,
    pub magical_def: i32,
}

pub struct SpecialData {
    pub effect: SpecialEffect,
}

pub enum SpecialEffect {
    Chest(ChestData),
    Title {
        title: &'static str,
        duration: TitleDuration,
    },
    Elixir {
        effect: ElixirEffect,
        duration_secs: u32,
    },
    InventoryExpansion {
        inventory_type: InventoryType,
        rows: i16,
    },
}

pub enum TitleDuration {
    Permanent,
    Temporary { secs: u32 },
}

pub enum ElixirEffect {
    ExpBoost { multiplier: f32 },
    DropRateBoost { multiplier: f32 },
    StatBuff { stat: &'static str, amount: i32 },
}

pub struct ChestData {
    pub contents: ChestContents,
}

pub enum ChestContents {
    Fixed(&'static [ChestFixedEntry]),
    Random {
        rolls: u32,
        pool: &'static [ChestPoolEntry],
    },
}

pub struct ChestFixedEntry {
    pub item_slug: &'static str,
    pub quantity: i32,
}

pub struct ChestPoolEntry {
    pub item_slug: &'static str,
    pub weight: u32,
    pub quantity_min: i32,
    pub quantity_max: i32,
}
