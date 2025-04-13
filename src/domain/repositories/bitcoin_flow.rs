use anyhow::Result;

use crate::domain::entities::bitcoin_ledger::{
    BitcoinBuyLedger, BitcoinSellLedger, BuyBitcoinDto, SellBitcoinDto,
};

#[async_trait::async_trait]
#[mockall::automock]
pub trait BitcoinFlowRepository {
    async fn record_buy(&self, buy_bitcoin_dto: BuyBitcoinDto) -> Result<i32>;
    async fn record_sell(&self, sell_bitcoin_dto: SellBitcoinDto) -> Result<i32>;
    async fn view_all_buy(&self) -> Result<Vec<BitcoinBuyLedger>>;
    async fn view_all_sell(&self) -> Result<Vec<BitcoinSellLedger>>;
}
