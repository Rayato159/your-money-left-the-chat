use anyhow::Result;

use crate::domain::entities::bitcoin_ledger::{BuyBitcoinDto, SellBitcoinDto};

#[async_trait::async_trait]
#[mockall::automock]
pub trait BitcoinFlowRepository {
    async fn buy(&self, buy_bitcoin_dto: BuyBitcoinDto) -> Result<i64>;
    async fn sell(&self, sell_bitcoin_dto: SellBitcoinDto) -> Result<i64>;
    async fn view_all_buy(&self) -> Result<Vec<BuyBitcoinDto>>;
    async fn view_all_sell(&self) -> Result<Vec<SellBitcoinDto>>;
}
