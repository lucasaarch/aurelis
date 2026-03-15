use crate::models::{
    combat_stats::FixedStatLine,
    equipment_slot::EquipmentSlot,
    inventory_type::InventoryType,
    item_instance_attributes::{EquipmentRollBias, StatModifierValueKind},
    item_rarity::ItemRarity,
    skill_data::CharacterSkillUnlockTier,
    stat_modifier::ModifierStat,
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
    Gem(GemData),
    Material,
    Consumable(ConsumableData),
    Special(SpecialData),
}

pub struct WeaponData {
    pub slot: EquipmentSlot,
    pub class: Option<&'static str>,
    pub level_req: i16,
    pub fixed_stats: &'static [FixedStatLine],
    pub fixed_special_effects: &'static [CatalogStatModifier],
    pub gem_slots: GemSlotConfig,
    pub identification: Option<EquipmentIdentificationRules>,
}

pub struct ArmorData {
    pub slot: EquipmentSlot,
    pub level_req: i16,
    pub fixed_stats: &'static [FixedStatLine],
    pub fixed_special_effects: &'static [CatalogStatModifier],
    pub gem_slots: GemSlotConfig,
    pub identification: Option<EquipmentIdentificationRules>,
}

pub struct ConsumableData {
    pub duration_secs: Option<u32>,
    pub hp_restore_percent: Option<f32>,
    pub mp_restore_percent: Option<f32>,
    pub power_boost_percent: Option<f32>,
    pub move_speed_boost_percent: Option<f32>,
    pub attack_speed_boost_percent: Option<f32>,
}

pub struct GemData {
    pub fixed_modifiers: &'static [CatalogStatModifier],
    pub effect_rolls: Option<GemEffectRollRules>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GemSlotConfig {
    pub base_slots: i16,
    pub max_bonus_slots: i16,
}

impl GemSlotConfig {
    pub const FOUR_BASE_PLUS_ONE_BONUS: Self = Self {
        base_slots: 4,
        max_bonus_slots: 1,
    };

    pub const fn total_possible_slots(self) -> i16 {
        self.base_slots + self.max_bonus_slots
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogStatModifier {
    pub id: &'static str,
    pub stat: ModifierStat,
    pub kind: StatModifierValueKind,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogStatModifierDefinition {
    pub id: &'static str,
    pub stat: ModifierStat,
    pub kind: StatModifierValueKind,
    pub min_value: i32,
    pub max_value: i32,
    pub weight: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentIdentificationRules {
    pub starts_unidentified: bool,
    pub bias: EquipmentRollBias,
    pub additional_effect_count: i16,
    pub additional_effect_pool: &'static [CatalogStatModifierDefinition],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GemEffectRollRules {
    pub roll_count: i16,
    pub effect_pool: &'static [CatalogStatModifierDefinition],
}

impl ItemKind {
    pub fn equipment_slot(&self) -> Option<EquipmentSlot> {
        match self {
            ItemKind::Weapon(data) => Some(data.slot),
            ItemKind::Armor(data) => Some(data.slot),
            _ => None,
        }
    }

    pub fn fixed_stats(&self) -> Option<&'static [FixedStatLine]> {
        match self {
            ItemKind::Weapon(data) => Some(data.fixed_stats),
            ItemKind::Armor(data) => Some(data.fixed_stats),
            _ => None,
        }
    }

    pub fn gem_slots(&self) -> Option<GemSlotConfig> {
        match self {
            ItemKind::Weapon(data) => Some(data.gem_slots),
            ItemKind::Armor(data) => Some(data.gem_slots),
            _ => None,
        }
    }

    pub fn fixed_special_effects(&self) -> Option<&'static [CatalogStatModifier]> {
        match self {
            ItemKind::Weapon(data) => Some(data.fixed_special_effects),
            ItemKind::Armor(data) => Some(data.fixed_special_effects),
            _ => None,
        }
    }

    pub fn gem_modifiers(&self) -> Option<&'static [CatalogStatModifier]> {
        match self {
            ItemKind::Gem(data) => Some(data.fixed_modifiers),
            _ => None,
        }
    }

    pub fn gem_effect_rolls(&self) -> Option<GemEffectRollRules> {
        match self {
            ItemKind::Gem(data) => data.effect_rolls,
            _ => None,
        }
    }

    pub fn identification(&self) -> Option<EquipmentIdentificationRules> {
        match self {
            ItemKind::Weapon(data) => data.identification,
            ItemKind::Armor(data) => data.identification,
            _ => None,
        }
    }
}

pub struct SpecialData {
    pub effect: SpecialEffect,
}

pub enum SpecialEffect {
    Chest(ChestData),
    CharacterSkillUnlock {
        tier: CharacterSkillUnlockTier,
    },
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
