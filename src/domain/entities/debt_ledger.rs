use crate::{
    domain::value_objects::debt_radar::DebtViewModel, infrastructure::database::schema::debt_ledger,
};
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = debt_ledger)]
pub struct DebtLedger {
    pub id: i64,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: String,
}

impl DebtLedger {
    pub fn to_model(&self) -> DebtViewModel {
        DebtViewModel {
            id: self.id,
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            who: self.who.to_owned(),
            date: self.date.to_owned().parse().unwrap(),
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
