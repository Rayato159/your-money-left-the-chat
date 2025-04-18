use anyhow::Result;

use crate::domain::entities::debt_ledger::{DebtLedger, PaidDebtLedgerDto, RecordDebtLedgerDto};

#[async_trait::async_trait]
#[mockall::automock]
pub trait DebtRadarRepository {
    async fn record(&self, record_debt_ledger_dto: RecordDebtLedgerDto) -> Result<i32>;
    async fn view_all(&self) -> Result<Vec<DebtLedger>>;
    async fn view_by_that_bro(&self, who: &str) -> Result<Vec<DebtLedger>>;
    async fn record_paid_debt(&self, paid_debt_ledger_dto: PaidDebtLedgerDto) -> Result<i32>;
}
