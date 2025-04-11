use rmcp::schemars;
use serde::{Deserialize, Serialize};

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
