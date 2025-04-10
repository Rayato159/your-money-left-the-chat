use crate::{
    domain::value_objects::spending_scanner::SpendingScannerModel,
    infrastructure::database::schema::my_ledger,
};
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = my_ledger)]
pub struct MyLedger {
    pub id: i64,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub date: String,
}

impl MyLedger {
    pub fn to_spending_scanner_model(&self) -> SpendingScannerModel {
        SpendingScannerModel {
            id: self.id,
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = my_ledger)]
pub struct RecordMyLedgerDto {
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub date: String,
}
