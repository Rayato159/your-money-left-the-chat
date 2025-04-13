use anyhow::Result;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::bitcoin_ledger::{
            BitcoinBuyLedger, BitcoinSellLedger, BuyBitcoinDto, SellBitcoinDto,
        },
        repositories::bitcoin_flow::BitcoinFlowRepository,
    },
    infrastructure::database::{
        SqlitePoolSquad,
        schema::{bitcoin_buy_ledger, bitcoin_sell_ledger},
    },
};

#[derive(Clone)]
pub struct BitcoinFlowSqlite {
    db_pool: Arc<SqlitePoolSquad>,
}

impl BitcoinFlowSqlite {
    pub fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl BitcoinFlowRepository for BitcoinFlowSqlite {
    async fn record_buy(&self, buy_bitcoin_dto: BuyBitcoinDto) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = diesel::insert_into(bitcoin_buy_ledger::table)
            .values(buy_bitcoin_dto)
            .returning(bitcoin_buy_ledger::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }

    async fn record_sell(&self, sell_bitcoin_dto: SellBitcoinDto) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = diesel::insert_into(bitcoin_sell_ledger::table)
            .values(sell_bitcoin_dto)
            .returning(bitcoin_sell_ledger::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }

    async fn view_all_buy(&self) -> Result<Vec<BitcoinBuyLedger>> {
        let conn = &mut self.db_pool.get()?;

        let results = bitcoin_buy_ledger::table
            .select(BitcoinBuyLedger::as_select())
            .load::<BitcoinBuyLedger>(conn)?;

        Ok(results)
    }

    async fn view_all_sell(&self) -> Result<Vec<BitcoinSellLedger>> {
        let conn = &mut self.db_pool.get()?;

        let results = bitcoin_sell_ledger::table
            .select(BitcoinSellLedger::as_select())
            .load::<BitcoinSellLedger>(conn)?;

        Ok(results)
    }
}
