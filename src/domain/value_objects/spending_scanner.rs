use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingScannerModel {
    pub id: i64,
    pub amount: f64,
    pub category: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
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
        start: chrono::NaiveDateTime,
        end: chrono::NaiveDateTime,
    },
}
