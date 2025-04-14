use rmcp::schemars;
use serde::{Deserialize, Serialize};

use crate::domain::entities::tax_deductions_list::AddTaxDeductionsListDto;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct TaxSimulateRequestModel {
    pub year: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxSimulateResult {
    pub must_pay: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AddTaxDeductionsListModel {
    pub title: String,
    pub amount: f32,
}

impl AddTaxDeductionsListModel {
    pub fn to_dto(&self) -> AddTaxDeductionsListDto {
        AddTaxDeductionsListDto {
            title: self.title.to_owned(),
            amount: self.amount,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RemoveTaxDeductionsListModel {
    pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxDeductionsListModel {
    pub id: i32,
    pub title: String,
    pub amount: f32,
}
