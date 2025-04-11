use std::sync::Arc;

use crate::domain::{
    repositories::cash_flow::CashFlowRepository,
    value_objects::cash_flow::{RecordCashFlowModel, RecordCashFlowWithDateModel},
};
use anyhow::Result;

#[derive(Clone)]
pub struct CashFlowUseCase {
    pub cash_flow_repository: Arc<dyn CashFlowRepository + Send + Sync + 'static>,
}

impl CashFlowUseCase {
    pub fn new(cash_flow_repository: Arc<dyn CashFlowRepository + Send + Sync + 'static>) -> Self {
        Self {
            cash_flow_repository,
        }
    }

    pub async fn record(&self, record_cash_flow_model: RecordCashFlowModel) -> Result<i32> {
        self.cash_flow_repository
            .record(record_cash_flow_model.to_dto())
            .await
    }

    pub async fn record_with_date(
        &self,
        record_cash_flow_with_date_model: RecordCashFlowWithDateModel,
    ) -> Result<i32> {
        self.cash_flow_repository
            .record(record_cash_flow_with_date_model.to_dto())
            .await
    }
}
