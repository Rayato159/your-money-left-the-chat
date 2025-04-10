use crate::domain::value_objects::spending_scanner::SpendingScannerModel;

#[derive(Debug, Clone)]
pub struct MyLedger {
    pub id: i64,
    pub amount: f64,
    pub category: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
}

impl MyLedger {
    pub fn to_spending_scanner_model(&self) -> SpendingScannerModel {
        SpendingScannerModel {
            id: self.id,
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            date: self.date,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordMyLedgerDto {
    pub amount: f64,
    pub category: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
}
