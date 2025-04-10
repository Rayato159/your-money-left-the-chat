use serde::{Deserialize, Serialize};

use crate::domain::entities::my_ledger::RecordMyLedgerDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordCashFlowModel {
    pub amount: f64,
    pub category: String,
    pub description: String,
}

impl RecordCashFlowModel {
    pub fn to_dto(&self) -> RecordMyLedgerDto {
        RecordMyLedgerDto {
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            date: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordCashFlowWithDateModel {
    pub amount: f64,
    pub category: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
}

impl RecordCashFlowWithDateModel {
    pub fn to_dto(&self) -> RecordMyLedgerDto {
        RecordMyLedgerDto {
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            date: self.date,
        }
    }
}
