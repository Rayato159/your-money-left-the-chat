use diesel::prelude::*;

use crate::{
    domain::value_objects::spending_scanner::MonthlySpendingModel,
    infrastructure::database::schema::monthly_spending,
};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = monthly_spending)]
pub struct MonthlySpending {
    pub id: i32,
    pub title: String,
    pub amount: f32,
    pub due_date: String,
}

impl MonthlySpending {
    pub fn to_model(&self) -> MonthlySpendingModel {
        MonthlySpendingModel {
            id: self.id,
            title: self.title.to_owned(),
            amount: self.amount,
            due_date: self.due_date.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = monthly_spending)]
pub struct AddMonthlySpendingDto {
    pub title: String,
    pub amount: f32,
    pub due_date: String,
}
