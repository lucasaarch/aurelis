use chrono::{NaiveDateTime, TimeZone, Utc};

pub fn format_naive_datetime(dt: &NaiveDateTime) -> String {
    Utc.from_utc_datetime(dt).to_rfc3339()
}
