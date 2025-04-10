use crate::domain::value_objects::debt_radar::DebtViewModel;

#[derive(Debug, Clone)]
pub struct DebtLedger {
    pub id: i64,
    pub amount: f64,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: chrono::NaiveDateTime,
}

impl DebtLedger {
    pub fn to_model(&self) -> DebtViewModel {
        DebtViewModel {
            id: self.id,
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            who: self.who.to_owned(),
            date: self.date,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordDebtLedgerDto {
    pub amount: f64,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: chrono::NaiveDateTime,
}
