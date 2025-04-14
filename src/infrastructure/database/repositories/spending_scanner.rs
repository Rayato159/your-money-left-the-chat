use anyhow::Result;
use chrono::Local;
use diesel::{
    dsl::{delete, insert_into},
    prelude::*,
};
use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            monthly_spending::{AddMonthlySpendingDto, MonthlySpending},
            my_ledger::MyLedger,
        },
        repositories::spending_scanner::SpendingScannerRepository,
    },
    infrastructure::database::{
        SqlitePoolSquad,
        schema::{monthly_spending, my_ledger},
    },
};

#[derive(Clone)]
pub struct SpendingScannerSqlite {
    db_pool: Arc<SqlitePoolSquad>,
}

impl SpendingScannerSqlite {
    pub fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl SpendingScannerRepository for SpendingScannerSqlite {
    async fn today(&self) -> Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;
        let today_prefix = Local::now().format("%Y-%m-%d").to_string(); // e.g. "2025-04-11"

        let result = my_ledger::table
            .filter(my_ledger::date.like(format!("{}%", today_prefix)))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn this_month(&self) -> Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;
        let this_month_prefix = Local::now().format("%Y-%m").to_string(); // e.g. "2025-04"

        let result = my_ledger::table
            .filter(my_ledger::date.like(format!("{}%", this_month_prefix)))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn this_year(&self) -> Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;
        let this_year_prefix = Local::now().format("%Y").to_string(); // e.g. "2025"

        let result = my_ledger::table
            .filter(my_ledger::date.like(format!("{}%", this_year_prefix)))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn lifetime(&self) -> Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;

        let result = my_ledger::table
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn custom(&self, start: String, end: String) -> Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;

        let result = my_ledger::table
            .filter(my_ledger::date.between(start, end))
            .order(my_ledger::date.desc())
            .load::<MyLedger>(conn)?;

        Ok(result)
    }

    async fn view_all_monthly_spending(&self) -> Result<Vec<MonthlySpending>> {
        let conn = &mut self.db_pool.get()?;

        let result = monthly_spending::table
            .order(monthly_spending::due_date.asc())
            .load::<MonthlySpending>(conn)?;

        Ok(result)
    }

    async fn add_monthly_spending(
        &self,
        add_monthly_spending_model: AddMonthlySpendingDto,
    ) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = insert_into(monthly_spending::table)
            .values(&add_monthly_spending_model)
            .returning(monthly_spending::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }

    async fn remove_monthly_spending(&self, id: i32) -> Result<()> {
        let conn = &mut self.db_pool.get()?;

        delete(monthly_spending::table)
            .filter(monthly_spending::id.eq(id))
            .execute(conn)?;

        Ok(())
    }
}
