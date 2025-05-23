use crate::{
    domain::value_objects::debt_radar::DebtViewModel, infrastructure::database::schema::debt_ledger,
};
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = debt_ledger)]
pub struct DebtLedger {
    pub id: i32,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: String,
}

impl DebtLedger {
    pub fn to_model(&self) -> DebtViewModel {
        DebtViewModel {
            amount: self.amount,
            who: self.who.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = debt_ledger)]
pub struct RecordDebtLedgerDto {
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: String,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = debt_ledger)]
pub struct PaidDebtLedgerDto {
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: String,
}
