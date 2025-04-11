use anyhow::Result;

use crate::domain::entities::my_ledger::MyLedger;

#[async_trait::async_trait]
#[mockall::automock]
pub trait SpendingScannerRepository {
    async fn today(&self) -> Result<Vec<MyLedger>>;
    async fn this_month(&self) -> Result<Vec<MyLedger>>;
    async fn this_year(&self) -> Result<Vec<MyLedger>>;
    async fn lifetime(&self) -> Result<Vec<MyLedger>>;
    async fn custom(&self, start: String, end: String) -> Result<Vec<MyLedger>>;
}
