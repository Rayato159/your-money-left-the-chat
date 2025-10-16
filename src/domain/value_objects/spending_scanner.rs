use rmcp::schemars;
use serde::{Deserialize, Serialize};

use crate::domain::entities::monthly_spending::AddMonthlySpendingDto;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SpendingScannerFilter {
    pub filter: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingScannerModel {
    pub id: i32,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Range {
    Today,
    ThisMonth,
    ThisYear,
    Lifetime,
    Custom { start: String, end: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AddMonthlySpendingModel {
    pub title: String,
    pub amount: f32,
    pub due_date: String, // DD
}

impl AddMonthlySpendingModel {
    pub fn to_dto(&self) -> AddMonthlySpendingDto {
        AddMonthlySpendingDto {
            title: self.title.to_owned(),
            amount: self.amount,
            due_date: self.due_date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RemoveMonthlySpendingModel {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct MonthlySpendingModel {
    pub id: i32,
    pub title: String,
    pub amount: f32,
    pub due_date: String,
}
