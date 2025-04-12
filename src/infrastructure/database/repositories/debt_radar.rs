use anyhow::Result;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::debt_ledger::{DebtLedger, PaidDebtLedgerDto, RecordDebtLedgerDto},
        repositories::debt_radar::DebtRadarRepository,
    },
    infrastructure::database::{SqlitePoolSquad, schema::debt_ledger},
};

#[derive(Clone)]
pub struct DebtRadarSqlite {
    db_pool: Arc<SqlitePoolSquad>,
}

impl DebtRadarSqlite {
    pub fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl DebtRadarRepository for DebtRadarSqlite {
    async fn record(&self, record_debt_ledger_dto: RecordDebtLedgerDto) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = diesel::insert_into(debt_ledger::table)
            .values(&record_debt_ledger_dto)
            .returning(debt_ledger::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }

    async fn view_all(&self) -> Result<Vec<DebtLedger>> {
        let conn = &mut self.db_pool.get()?;

        let results = debt_ledger::table
            .select(DebtLedger::as_select())
            .load::<DebtLedger>(conn)?;

        Ok(results)
    }

    async fn view_by_that_bro(&self, who: &str) -> Result<Vec<DebtLedger>> {
        let conn = &mut self.db_pool.get()?;

        let results = debt_ledger::table
            .filter(debt_ledger::who.eq(who))
            .select(DebtLedger::as_select())
            .load::<DebtLedger>(conn)?;

        Ok(results)
    }

    async fn record_paid_debt(&self, paid_debt_ledger_dto: PaidDebtLedgerDto) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = diesel::insert_into(debt_ledger::table)
            .values(&paid_debt_ledger_dto)
            .returning(debt_ledger::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }
}
