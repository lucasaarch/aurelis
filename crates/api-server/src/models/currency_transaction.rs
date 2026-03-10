use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::currency_transaction::CurrencyTransaction;
use uuid::Uuid;

use crate::models::currency_origin::CurrencyOriginModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::currency_transactions)]
pub struct CurrencyTransactionModel {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub character_id: Option<Uuid>,
    pub currency: String,
    pub amount: i64,
    pub balance_after: i64,
    pub origin: CurrencyOriginModel,
    pub reference_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}

impl CurrencyTransactionModel {
    pub fn new(
        account_id: Option<Uuid>,
        character_id: Option<Uuid>,
        currency: String,
        amount: i64,
        balance_after: i64,
        origin: CurrencyOriginModel,
        reference_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id,
            character_id,
            currency,
            amount,
            balance_after,
            origin,
            reference_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}

impl From<CurrencyTransactionModel> for CurrencyTransaction {
    fn from(model: CurrencyTransactionModel) -> Self {
        Self {
            id: model.id,
            account_id: model.account_id,
            character_id: model.character_id,
            currency: model.currency,
            amount: model.amount,
            balance_after: model.balance_after,
            origin: model.origin.into(),
            reference_id: model.reference_id,
            created_at: model.created_at,
        }
    }
}
