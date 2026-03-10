use crate::models::{chat_restriction_severity::ChatRestrictionSeverity, suspension_severity::SuspensionSeverity};


pub enum Punishment {
    Ban,
    Mute(ChatRestrictionSeverity),
    Suspension(SuspensionSeverity),
}