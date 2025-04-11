use anyhow::Result;
use diesel::{RunQueryDsl, dsl::insert_into};
use std::sync::Arc;

use crate::{
    domain::{entities::my_ledger::RecordMyLedgerDto, repositories::cash_flow::CashFlowRepository},
    infrastructure::database::{SqlitePoolSquad, schema::my_ledger},
};

pub struct CashFlowSQLite {
    pub db_pool: Arc<SqlitePoolSquad>,
}

impl CashFlowSQLite {
    pub async fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl CashFlowRepository for CashFlowSQLite {
    async fn record(&self, record_my_ledger_dto: RecordMyLedgerDto) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = insert_into(my_ledger::table)
            .values(record_my_ledger_dto)
            .returning(my_ledger::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }
}
