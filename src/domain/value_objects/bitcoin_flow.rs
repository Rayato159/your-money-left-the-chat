use rmcp::schemars;
use serde::{Deserialize, Serialize};

use crate::domain::entities::bitcoin_ledger::{BuyBitcoinDto, SellBitcoinDto};

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BuyBitcoin {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

impl BuyBitcoin {
    pub fn to_dto(&self) -> BuyBitcoinDto {
        BuyBitcoinDto {
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellBitcoin {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

impl SellBitcoin {
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
pub struct ViewBitcoinLedger {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub operation: String,
    pub date: String,
}
