use chrono::Local;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::my_ledger::MyLedger, repositories::spending_scanner::SpendingScannerRepository,
    },
    infrastructure::database::{SqlitePoolSquad, schema::my_ledger},
};

#[derive(Clone)]
pub struct SpendingScannerSqlite {
    pub db_pool: Arc<SqlitePoolSquad>,
}

impl SpendingScannerSqlite {
    pub fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl SpendingScannerRepository for SpendingScannerSqlite {
    async fn today(&self) -> anyhow::Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;
        let today_prefix = Local::now().format("%Y-%m-%d").to_string(); // e.g. "2025-04-11"

        let result = my_ledger::table
            .filter(my_ledger::date.like(format!("{}%", today_prefix)))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn this_month(&self) -> anyhow::Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;
        let this_month_prefix = Local::now().format("%Y-%m").to_string(); // e.g. "2025-04"

        let result = my_ledger::table
            .filter(my_ledger::date.like(format!("{}%", this_month_prefix)))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn this_year(&self) -> anyhow::Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;
        let this_year_prefix = Local::now().format("%Y").to_string(); // e.g. "2025"

        let result = my_ledger::table
            .filter(my_ledger::date.like(format!("{}%", this_year_prefix)))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn lifetime(&self) -> anyhow::Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;

        let result = my_ledger::table
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn custom(&self, start: String, end: String) -> anyhow::Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;

        let result = my_ledger::table
            .filter(my_ledger::date.between(start, end))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }
}
