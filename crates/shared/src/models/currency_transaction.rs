use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::currency_origin::CurrencyOrigin;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyTransaction {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub character_id: Option<Uuid>,
    pub currency: String,
    pub amount: i64,
    pub balance_after: i64,
    pub origin: CurrencyOrigin,
    pub reference_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}
