use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingScannerModel {
    pub id: i64,
    pub amount: f32,
    pub category: String,
    pub description: Option<String>,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SpendingScannerModelFilter {
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
    Lifetime,
    Custom {
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    },
}
