use diesel::{dsl::delete, prelude::*};
use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            my_ledger::MyLedger,
            tax_deductions_list::{AddTaxDeductionsListDto, TaxDeductionsList},
        },
        repositories::tax_simulator::TaxSimulatorRepository,
    },
    infrastructure::database::{
        SqlitePoolSquad,
        schema::{my_ledger, tax_deductions_list},
    },
};
use anyhow::Result;
use diesel::TextExpressionMethods;

#[derive(Clone)]
pub struct TaxSimulatorSqlite {
    db_pool: Arc<SqlitePoolSquad>,
}

impl TaxSimulatorSqlite {
    pub fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl TaxSimulatorRepository for TaxSimulatorSqlite {
    async fn view_all_income_by_year(&self, year: i32) -> Result<Vec<MyLedger>> {
        let conn = &mut self.db_pool.get()?;

        let results = my_ledger::table
            .filter(my_ledger::amount.gt(0.0))
            .filter(my_ledger::date.like(format!("{}%", year)))
            .order(my_ledger::date.desc())
            .select(MyLedger::as_select())
            .load::<MyLedger>(conn)?;

        Ok(results)
    }

    async fn view_all_tax_deductions_list(&self) -> Result<Vec<TaxDeductionsList>> {
        let conn = &mut self.db_pool.get()?;

        let results = tax_deductions_list::table
            .order(tax_deductions_list::id.desc())
            .select(TaxDeductionsList::as_select())
            .load::<TaxDeductionsList>(conn)?;

        Ok(results)
    }

    async fn add_tax_deduction_list(
        &self,
        add_tax_deduction_list_dto: AddTaxDeductionsListDto,
    ) -> Result<i32> {
        let conn = &mut self.db_pool.get()?;

        let result_id = diesel::insert_into(tax_deductions_list::table)
            .values(&add_tax_deduction_list_dto)
            .returning(tax_deductions_list::id)
            .get_result::<i32>(conn)?;

        Ok(result_id)
    }

    async fn remove_tax_deduction_list(&self, id: i32) -> Result<()> {
        let conn = &mut self.db_pool.get()?;

        delete(tax_deductions_list::table)
            .filter(tax_deductions_list::id.eq(id))
            .execute(conn)?;

        Ok(())
    }
}
