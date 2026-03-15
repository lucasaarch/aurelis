use crate::models::combat_stats::CombatStats;

pub struct CharacterData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub base_stats: CharacterBaseStats,
    pub evolution_lines: &'static [&'static ClassPathData],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterBaseStats {
    pub hp: i32,
    pub mp: i32,
    pub physical_atk: i32,
    pub magical_atk: i32,
    pub physical_def: i32,
    pub magical_def: i32,
    pub move_spd: i32,
    pub atk_spd: i32,
}

pub struct ClassPathData {
    pub steps: &'static [&'static ClassData],
}

pub struct ClassData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub class_type: ClassType,
    pub affinity: CombatAffinity,
    pub stat_bonuses: CharacterBaseStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatAffinity {
    Neutral,
    Physical,
    Magical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassType {
    First,
    Second,
}

impl ClassType {
    pub fn level_req(&self) -> i16 {
        match self {
            ClassType::First => 15,
            ClassType::Second => 35,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterClassTier {
    Base,
    First,
    Second,
}

pub struct CharacterProgress {
    pub selected_path_index: usize,
    pub tier: CharacterClassTier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CharacterSkillUnlocks {
    pub beginner: bool,
    pub intermediate: bool,
}

impl CharacterSkillUnlocks {
    pub fn has_tier(&self, tier: crate::models::skill_data::CharacterSkillUnlockTier) -> bool {
        match tier {
            crate::models::skill_data::CharacterSkillUnlockTier::Beginner => self.beginner,
            crate::models::skill_data::CharacterSkillUnlockTier::Intermediate => self.intermediate,
        }
    }
}

impl CharacterData {
    pub fn find_path(&self, index: usize) -> Option<&'static ClassPathData> {
        self.evolution_lines.get(index).copied()
    }

    pub fn unlocked_class_slugs<'a>(
        &'a self,
        progress: &CharacterProgress,
    ) -> Option<Vec<&'a str>> {
        let path = self.find_path(progress.selected_path_index)?;
        let first = path
            .steps
            .iter()
            .copied()
            .find(|step| step.class_type == ClassType::First)?;
        let second = path
            .steps
            .iter()
            .copied()
            .find(|step| step.class_type == ClassType::Second)?;

        let mut slugs = vec![self.slug];
        match progress.tier {
            CharacterClassTier::Base => {}
            CharacterClassTier::First => {
                slugs.push(first.slug);
            }
            CharacterClassTier::Second => {
                slugs.push(first.slug);
                slugs.push(second.slug);
            }
        }
        Some(slugs)
    }

    pub fn find_class_by_slug(&self, class_slug: &str) -> Option<&'static ClassData> {
        self.evolution_lines
            .iter()
            .flat_map(|path| path.steps.iter().copied())
            .find(|class| class.slug == class_slug)
    }

    pub fn unlocked_class_slugs_for_current_class<'a>(
        &'a self,
        current_class_slug: &str,
    ) -> Option<Vec<&'a str>> {
        if self.slug == current_class_slug {
            return Some(vec![self.slug]);
        }

        for path in self.evolution_lines {
            let mut slugs = vec![self.slug];
            for class in path.steps {
                slugs.push(class.slug);
                if class.slug == current_class_slug {
                    return Some(slugs);
                }
            }
        }

        None
    }

    pub fn affinity_for_current_class(&self, current_class_slug: &str) -> Option<CombatAffinity> {
        if self.slug == current_class_slug {
            return Some(CombatAffinity::Neutral);
        }

        self.find_class_by_slug(current_class_slug)
            .map(|class| class.affinity)
    }
}

impl From<CharacterBaseStats> for CombatStats {
    fn from(value: CharacterBaseStats) -> Self {
        Self {
            core: crate::models::combat_stats::CombatCoreStats {
                hp: value.hp,
                mp: value.mp,
                physical_atk: value.physical_atk,
                magical_atk: value.magical_atk,
                physical_def: value.physical_def,
                magical_def: value.magical_def,
                move_spd: value.move_spd,
                atk_spd: value.atk_spd,
            },
            secondary: crate::models::combat_stats::CombatSecondaryStats {
                damage_reduction: 0,
                crit_chance: 0,
                crit_damage: 0,
                accuracy: 0,
                physical_attack_level: 0,
                magical_attack_level: 0,
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
        }
    }
}
