use rmcp::schemars;
use serde::{Deserialize, Serialize};

use crate::domain::entities::debt_ledger::{PaidDebtLedgerDto, RecordDebtLedgerDto};

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WhoOwesMeModel {
    pub who: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RecordDebtModel {
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub who: String,
}

impl RecordDebtModel {
    pub fn to_dto(&self) -> RecordDebtLedgerDto {
        RecordDebtLedgerDto {
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            who: self.who.to_owned(),
            date: chrono::Utc::now().naive_utc().date().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct PaidDebtModel {
    pub amount: f32,
    pub who: String,
    pub date: String,
}

impl PaidDebtModel {
    pub fn to_dto(&self) -> PaidDebtLedgerDto {
        PaidDebtLedgerDto {
            amount: -self.amount,
            category: "Paid Debt".to_string(),
            description: "Paid Debt".to_string(),
            who: self.who.to_owned(),
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDebtWithDateModel {
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub who: String,
    pub date: String,
}

impl RecordDebtWithDateModel {
    pub fn to_dto(&self) -> RecordDebtLedgerDto {
        RecordDebtLedgerDto {
            amount: self.amount,
            category: self.category.to_owned(),
            description: self.description.to_owned(),
            who: self.who.to_owned(),
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtViewModel {
    pub amount: f32,
    pub who: String,
}
