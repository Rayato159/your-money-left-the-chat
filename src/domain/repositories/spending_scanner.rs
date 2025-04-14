use anyhow::Result;

use crate::domain::entities::{
    monthly_spending::{AddMonthlySpendingDto, MonthlySpending},
    my_ledger::MyLedger,
};

#[async_trait::async_trait]
#[mockall::automock]
pub trait SpendingScannerRepository {
    async fn today(&self) -> Result<Vec<MyLedger>>;
    async fn this_month(&self) -> Result<Vec<MyLedger>>;
    async fn this_year(&self) -> Result<Vec<MyLedger>>;
    async fn lifetime(&self) -> Result<Vec<MyLedger>>;
    async fn custom(&self, start: String, end: String) -> Result<Vec<MyLedger>>;
    async fn view_all_monthly_spending(&self) -> Result<Vec<MonthlySpending>>;
    async fn add_monthly_spending(
        &self,
        add_monthly_spending_model: AddMonthlySpendingDto,
    ) -> Result<i32>;
    async fn remove_monthly_spending(&self, id: i32) -> Result<()>;
}
