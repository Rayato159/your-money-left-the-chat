use anyhow::Result;

use crate::domain::entities::my_ledger::RecordMyLedgerDto;

#[async_trait::async_trait]
#[mockall::automock]
pub trait CashFlowRepository {
    async fn record(&self, record_my_ledger_dto: RecordMyLedgerDto) -> Result<i32>;
}
