use anyhow::Result;

use crate::domain::entities::debt_ledger::{DebtLedger, RecordDebtLedgerDto};

#[async_trait::async_trait]
#[mockall::automock]
pub trait DebtRadarRepository {
    async fn record(&self, record_debt_ledger_dto: RecordDebtLedgerDto) -> Result<i64>;
    async fn view_all(&self) -> Result<Vec<DebtLedger>>;
    async fn view_by_that_bro(&self, who: String) -> Result<Vec<DebtLedger>>;
}
