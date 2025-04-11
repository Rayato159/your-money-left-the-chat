use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingScannerModel {
    pub id: i32,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub date: String,
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
        start: String,
        end: String,
    },
}
