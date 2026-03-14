#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatKey {
    Hp,
    Mp,
    PhysicalAtk,
    MagicalAtk,
    PhysicalDef,
    MagicalDef,
    MoveSpd,
    AtkSpd,
    DamageReduction,
    CritChance,
    CritDamage,
    Accuracy,
    PhysicalPen,
    MagicalPen,
    HpRegen,
    MpRegen,
    LifeSteal,
    CooldownReduction,
    CritResistance,
    KnockbackResistance,
    CcResistance,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedStatLine {
    pub stat: StatKey,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatCoreStats {
    pub hp: i32,
    pub mp: i32,
    pub physical_atk: i32,
    pub magical_atk: i32,
    pub physical_def: i32,
    pub magical_def: i32,
    pub move_spd: i32,
    pub atk_spd: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatSecondaryStats {
    pub damage_reduction: i32,
    pub crit_chance: i32,
    pub crit_damage: i32,
    pub accuracy: i32,
    pub physical_pen: i32,
    pub magical_pen: i32,
    pub hp_regen: i32,
    pub mp_regen: i32,
    pub life_steal: i32,
    pub cooldown_reduction: i32,
    pub crit_resistance: i32,
    pub knockback_resistance: i32,
    pub cc_resistance: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatStats {
    pub core: CombatCoreStats,
    pub secondary: CombatSecondaryStats,
}

impl CombatStats {
    pub const ZERO: Self = Self {
        core: CombatCoreStats {
            hp: 0,
            mp: 0,
            physical_atk: 0,
            magical_atk: 0,
            physical_def: 0,
            magical_def: 0,
            move_spd: 0,
            atk_spd: 0,
        },
        secondary: CombatSecondaryStats {
            damage_reduction: 0,
            crit_chance: 0,
            crit_damage: 0,
            accuracy: 0,
            physical_pen: 0,
            magical_pen: 0,
            hp_regen: 0,
            mp_regen: 0,
            life_steal: 0,
            cooldown_reduction: 0,
            crit_resistance: 0,
            knockback_resistance: 0,
            cc_resistance: 0,
        },
    };

    pub fn add_line(&mut self, line: FixedStatLine) {
        match line.stat {
            StatKey::Hp => self.core.hp += line.value,
            StatKey::Mp => self.core.mp += line.value,
            StatKey::PhysicalAtk => self.core.physical_atk += line.value,
            StatKey::MagicalAtk => self.core.magical_atk += line.value,
            StatKey::PhysicalDef => self.core.physical_def += line.value,
            StatKey::MagicalDef => self.core.magical_def += line.value,
            StatKey::MoveSpd => self.core.move_spd += line.value,
            StatKey::AtkSpd => self.core.atk_spd += line.value,
            StatKey::DamageReduction => self.secondary.damage_reduction += line.value,
            StatKey::CritChance => self.secondary.crit_chance += line.value,
            StatKey::CritDamage => self.secondary.crit_damage += line.value,
            StatKey::Accuracy => self.secondary.accuracy += line.value,
            StatKey::PhysicalPen => self.secondary.physical_pen += line.value,
            StatKey::MagicalPen => self.secondary.magical_pen += line.value,
            StatKey::HpRegen => self.secondary.hp_regen += line.value,
            StatKey::MpRegen => self.secondary.mp_regen += line.value,
            StatKey::LifeSteal => self.secondary.life_steal += line.value,
            StatKey::CooldownReduction => self.secondary.cooldown_reduction += line.value,
            StatKey::CritResistance => self.secondary.crit_resistance += line.value,
            StatKey::KnockbackResistance => self.secondary.knockback_resistance += line.value,
            StatKey::CcResistance => self.secondary.cc_resistance += line.value,
        }
    }

    pub fn add_lines(&mut self, lines: &[FixedStatLine]) {
        for line in lines {
            self.add_line(*line);
        }
    }
}

impl std::ops::AddAssign for CombatStats {
    fn add_assign(&mut self, rhs: Self) {
        self.core.hp += rhs.core.hp;
        self.core.mp += rhs.core.mp;
        self.core.physical_atk += rhs.core.physical_atk;
        self.core.magical_atk += rhs.core.magical_atk;
        self.core.physical_def += rhs.core.physical_def;
        self.core.magical_def += rhs.core.magical_def;
        self.core.move_spd += rhs.core.move_spd;
        self.core.atk_spd += rhs.core.atk_spd;
        self.secondary.damage_reduction += rhs.secondary.damage_reduction;
        self.secondary.crit_chance += rhs.secondary.crit_chance;
        self.secondary.crit_damage += rhs.secondary.crit_damage;
        self.secondary.accuracy += rhs.secondary.accuracy;
        self.secondary.physical_pen += rhs.secondary.physical_pen;
        self.secondary.magical_pen += rhs.secondary.magical_pen;
        self.secondary.hp_regen += rhs.secondary.hp_regen;
        self.secondary.mp_regen += rhs.secondary.mp_regen;
        self.secondary.life_steal += rhs.secondary.life_steal;
        self.secondary.cooldown_reduction += rhs.secondary.cooldown_reduction;
        self.secondary.crit_resistance += rhs.secondary.crit_resistance;
        self.secondary.knockback_resistance += rhs.secondary.knockback_resistance;
        self.secondary.cc_resistance += rhs.secondary.cc_resistance;
    }
}
