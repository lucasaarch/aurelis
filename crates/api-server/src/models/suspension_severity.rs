use chrono::{Duration, NaiveDateTime, Utc};

pub enum SuspensionSeverity {
    Low,
    Medium,
    High,
}

impl SuspensionSeverity {
    pub fn duration(&self) -> i64 {
        match self {
            SuspensionSeverity::Low => 60 * 60 * 24,        // 1 day
            SuspensionSeverity::Medium => 60 * 60 * 24 * 7, // 1 week
            SuspensionSeverity::High => 60 * 60 * 24 * 30,  // 1 month
        }
    }
}

impl From<SuspensionSeverity> for NaiveDateTime {
    fn from(severity: SuspensionSeverity) -> Self {
        let now = Utc::now().naive_utc();
        now + Duration::seconds(severity.duration())
    }
}
