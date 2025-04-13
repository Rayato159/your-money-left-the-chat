use diesel::prelude::Queryable;
use diesel::prelude::*;

use crate::{
    domain::value_objects::bitcoin_flow::ViewBitcoinLedgerModel,
    infrastructure::database::schema::{bitcoin_buy_ledger, bitcoin_sell_ledger},
};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = bitcoin_sell_ledger)]
pub struct BitcoinSellLedger {
    pub id: i32,
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

impl BitcoinSellLedger {
    pub fn to_model(&self) -> ViewBitcoinLedgerModel {
        ViewBitcoinLedgerModel {
            id: self.id,
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            operation: "Sell".to_string(),
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = bitcoin_buy_ledger)]
pub struct BuyBitcoinDto {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = bitcoin_buy_ledger)]
pub struct BitcoinBuyLedger {
    pub id: i32,
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

impl BitcoinBuyLedger {
    pub fn to_model(&self) -> ViewBitcoinLedgerModel {
        ViewBitcoinLedgerModel {
            id: self.id,
            amount: self.amount,
            price: self.price,
            cost: self.cost,
            operation: "Buy".to_string(),
            date: self.date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = bitcoin_sell_ledger)]
pub struct SellBitcoinDto {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}
