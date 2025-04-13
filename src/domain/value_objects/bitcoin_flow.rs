use rmcp::schemars;
use serde::{Deserialize, Serialize};

use crate::domain::entities::bitcoin_ledger::{BuyBitcoinDto, SellBitcoinDto};

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BuyBitcoinModel {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

impl BuyBitcoinModel {
    pub fn to_dto(&self) -> BuyBitcoinDto {
        BuyBitcoinDto {
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SellBitcoinModel {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

impl SellBitcoinModel {
    pub fn to_dto(&self) -> SellBitcoinDto {
        SellBitcoinDto {
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ViewBitcoinLedgerModel {
    pub id: i32,
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub operation: String,
    pub date: String,
}
