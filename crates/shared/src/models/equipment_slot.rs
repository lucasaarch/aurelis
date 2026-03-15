use serde::{Deserialize, Serialize};

use crate::models::combat_stats::{CombatStats, StatKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Head,
    Chest,
    Legs,
    Gloves,
    Shoes,
    AccRing1,
    AccRing2,
    AccNecklace,
    AccEarrings,
    AccArm,
    AccFaceBottom,
    AccFaceMiddle,
    AccFaceTop,
    AccBottomPiece,
    AccTopPiece,
    AccWeapon,
    AccSupportUnit,
}

impl EquipmentSlot {
    pub fn calculate_refinement_bonus(
        self,
        refinement: i16,
        base_stats: &CombatStats,
    ) -> CombatStats {
        let refinement = refinement.clamp(0, 7);
        let mut bonus = CombatStats::ZERO;

        for stat in [
            StatKey::Hp,
            StatKey::Mp,
            StatKey::PhysicalAtk,
            StatKey::MagicalAtk,
            StatKey::PhysicalDef,
            StatKey::MagicalDef,
            StatKey::MoveSpd,
            StatKey::AtkSpd,
            StatKey::DamageReduction,
            StatKey::CritChance,
            StatKey::CritDamage,
            StatKey::Accuracy,
            StatKey::PhysicalAttackLevel,
            StatKey::MagicalAttackLevel,
            StatKey::PhysicalPen,
            StatKey::MagicalPen,
            StatKey::HpRegen,
            StatKey::MpRegen,
            StatKey::LifeSteal,
            StatKey::CooldownReduction,
            StatKey::CritResistance,
            StatKey::KnockbackResistance,
            StatKey::CcResistance,
        ] {
            let value_bp = i32::from(refinement) * self.refinement_percent_per_level_bp(stat);
            if value_bp == 0 {
                continue;
            }

            let base_value = base_stats.get_stat(stat);
            let delta = (base_value * value_bp) / 10_000;
            if delta != 0 {
                bonus.add_to_stat(stat, delta);
            }
        }

        bonus
    }

    pub fn refinement_percent_per_level_bp(self, stat: StatKey) -> i32 {
        match self {
            EquipmentSlot::Weapon => match stat {
                StatKey::PhysicalAtk | StatKey::MagicalAtk => 1000,
                StatKey::PhysicalAttackLevel | StatKey::MagicalAttackLevel => 800,
                StatKey::CritChance | StatKey::CritDamage | StatKey::Accuracy => 600,
                StatKey::Hp | StatKey::Mp => 400,
                _ => 0,
            },
            EquipmentSlot::Chest => match stat {
                StatKey::Hp => 800,
                StatKey::PhysicalDef | StatKey::MagicalDef => 700,
                StatKey::CritResistance | StatKey::KnockbackResistance | StatKey::CcResistance => {
                    500
                }
                _ => 0,
            },
            EquipmentSlot::Legs => match stat {
                StatKey::DamageReduction => 600,
                StatKey::PhysicalDef | StatKey::MagicalDef => 700,
                StatKey::Hp => 500,
                _ => 0,
            },
            EquipmentSlot::Gloves => match stat {
                StatKey::AtkSpd => 1000,
                StatKey::PhysicalDef | StatKey::MagicalDef => 600,
                StatKey::Accuracy => 700,
                _ => 0,
            },
            EquipmentSlot::Shoes => match stat {
                StatKey::MoveSpd => 300,
                StatKey::PhysicalDef | StatKey::MagicalDef => 600,
                StatKey::KnockbackResistance => 500,
                _ => 0,
            },
            EquipmentSlot::Head
            | EquipmentSlot::AccRing1
            | EquipmentSlot::AccRing2
            | EquipmentSlot::AccNecklace
            | EquipmentSlot::AccEarrings
            | EquipmentSlot::AccArm
            | EquipmentSlot::AccFaceBottom
            | EquipmentSlot::AccFaceMiddle
            | EquipmentSlot::AccFaceTop
            | EquipmentSlot::AccBottomPiece
            | EquipmentSlot::AccTopPiece
            | EquipmentSlot::AccWeapon
            | EquipmentSlot::AccSupportUnit => 0,
        }
    }
}
