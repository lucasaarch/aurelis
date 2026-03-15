use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewardStatKey {
    ExperienceGain,
    DropRate,
    CreditGain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewardStats {
    pub experience_gain: i32,
    pub drop_rate: i32,
    pub credit_gain: i32,
}

impl RewardStats {
    pub const ZERO: Self = Self {
        experience_gain: 0,
        drop_rate: 0,
        credit_gain: 0,
    };

    pub fn get_stat(&self, stat: RewardStatKey) -> i32 {
        match stat {
            RewardStatKey::ExperienceGain => self.experience_gain,
            RewardStatKey::DropRate => self.drop_rate,
            RewardStatKey::CreditGain => self.credit_gain,
        }
    }

    pub fn add_to_stat(&mut self, stat: RewardStatKey, value: i32) {
        match stat {
            RewardStatKey::ExperienceGain => self.experience_gain += value,
            RewardStatKey::DropRate => self.drop_rate += value,
            RewardStatKey::CreditGain => self.credit_gain += value,
        }
    }

    pub fn add_percent_to_stat(&mut self, stat: RewardStatKey, value_bp: i32) {
        let base_value = self.get_stat(stat);
        let delta = (base_value * value_bp) / 10_000;
        self.add_to_stat(stat, delta);
    }
}

impl std::ops::AddAssign for RewardStats {
    fn add_assign(&mut self, rhs: Self) {
        self.experience_gain += rhs.experience_gain;
        self.drop_rate += rhs.drop_rate;
        self.credit_gain += rhs.credit_gain;
    }
}
