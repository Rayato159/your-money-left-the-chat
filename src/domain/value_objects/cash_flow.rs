use rmcp::schemars;
use serde::{Deserialize, Serialize};

use crate::domain::entities::my_ledger::RecordMyLedgerDto;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RecordCashFlowModel {
    pub amount: f32,
    pub category: String,
    pub description: String,
}

impl RecordCashFlowModel {
    pub fn to_dto(&self) -> RecordMyLedgerDto {
        RecordMyLedgerDto {
            amount: self.amount,
            category: self.category.to_owned().to_uppercase(),
            description: self.description.to_owned(),
            date: chrono::Utc::now().naive_utc().date().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RecordCashFlowWithDateModel {
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub date: String,
}

impl RecordCashFlowWithDateModel {
    pub fn to_dto(&self) -> RecordMyLedgerDto {
        RecordMyLedgerDto {
            amount: self.amount,
            category: self.category.to_owned().to_uppercase(),
            description: self.description.to_owned(),
            date: self.date.to_string(),
        }
    }
}
