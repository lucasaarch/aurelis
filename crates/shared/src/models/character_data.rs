pub struct CharacterData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub base_stats: CharacterBaseStats,
    pub evolution_lines: &'static [&'static ClassPathData],
}

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
    pub stat_bonuses: CharacterBaseStats,
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
}
