use diesel::prelude::*;

use crate::{
    domain::value_objects::tax_simulator::TaxDeductionsListModel,
    infrastructure::database::schema::tax_deductions_list,
};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = tax_deductions_list)]
pub struct TaxDeductionsList {
    pub id: i32,
    pub title: String,
    pub amount: f32,
}

impl TaxDeductionsList {
    pub fn to_model(&self) -> TaxDeductionsListModel {
        TaxDeductionsListModel {
            id: self.id,
            title: self.title.to_owned(),
            amount: self.amount,
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = tax_deductions_list)]
pub struct AddTaxDeductionsListDto {
    pub title: String,
    pub amount: f32,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = tax_deductions_list)]
pub struct RemoveTaxDeductionsListDto {
    pub id: i32,
}
