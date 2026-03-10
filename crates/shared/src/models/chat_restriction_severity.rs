use chrono::{Duration, NaiveDateTime, Utc};

pub enum ChatRestrictionSeverity {
    Low,
    Medium,
    High,
}

impl ChatRestrictionSeverity {
    pub fn duration(&self) -> i64 {
        match self {
            ChatRestrictionSeverity::Low => 60 * 60, // 1 hour
            ChatRestrictionSeverity::Medium => 60 * 60 * 24, // 1 day
            ChatRestrictionSeverity::High => 60 * 60 * 24 * 7, // 1 week
        }
    }
}

impl From<ChatRestrictionSeverity> for NaiveDateTime {
    fn from(severity: ChatRestrictionSeverity) -> Self {
        let now = Utc::now().naive_utc();
        now + Duration::seconds(severity.duration())
    }
}