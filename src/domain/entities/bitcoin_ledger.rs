use diesel::prelude::Queryable;
use diesel::prelude::*;

use crate::infrastructure::database::schema::{bitcoin_buy_ledger, bitcoin_sell_ledger};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = bitcoin_buy_ledger)]
pub struct BitcoinSellLedger {
    pub id: i64,
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
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
#[diesel(table_name = bitcoin_sell_ledger)]
pub struct BitcoinBuyLedger {
    pub id: i64,
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = bitcoin_sell_ledger)]
pub struct SellBitcoinDto {
    pub amount: f32,
    pub price: f32,
    pub cost: f32,
    pub date: String,
}
