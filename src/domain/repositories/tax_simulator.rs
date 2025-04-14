use crate::domain::entities::{
    my_ledger::MyLedger,
    tax_deductions_list::{AddTaxDeductionsListDto, TaxDeductionsList},
};
use anyhow::Result;

#[async_trait::async_trait]
#[mockall::automock]
pub trait TaxSimulatorRepository {
    async fn view_all_income_by_year(&self, year: i32) -> Result<Vec<MyLedger>>;
    async fn view_all_tax_deductions_list(&self) -> Result<Vec<TaxDeductionsList>>;
    async fn add_tax_deduction_list(
        &self,
        add_tax_deduction_list_dto: AddTaxDeductionsListDto,
    ) -> Result<i32>;
    async fn remove_tax_deduction_list(&self, id: i32) -> Result<()>;
}
