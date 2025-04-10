use serde::{Deserialize, Serialize};

use crate::domain::entities::bitcoin_ledger::{BuyBitcoinDto, SellBitcoinDto};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyBitcoin {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: chrono::NaiveDate,
}

impl BuyBitcoin {
    pub fn to_dto(&self) -> BuyBitcoinDto {
        BuyBitcoinDto {
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            date: self.date.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellBitcoin {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: chrono::NaiveDate,
}

impl SellBitcoin {
    pub fn to_dto(&self) -> SellBitcoinDto {
        SellBitcoinDto {
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            date: self.date.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBitcoinLedger {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub operation: String,
    pub date: chrono::NaiveDate,
}
