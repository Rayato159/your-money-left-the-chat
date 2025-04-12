use std::sync::Arc;

use crate::domain::{
    repositories::cash_flow::CashFlowRepository,
    value_objects::cash_flow::{RecordCashFlowModel, RecordCashFlowWithDateModel},
};
use anyhow::Result;

#[derive(Clone)]
pub struct CashFlowUseCase {
    cash_flow_repository: Arc<dyn CashFlowRepository + Send + Sync + 'static>,
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::cash_flow::CashFlowUseCase,
        domain::{
            repositories::cash_flow::MockCashFlowRepository,
            value_objects::cash_flow::{RecordCashFlowModel, RecordCashFlowWithDateModel},
        },
    };

    #[tokio::test]
    async fn test_record_success() {
        let mut mock_cash_flow_repository = MockCashFlowRepository::new();

        mock_cash_flow_repository
            .expect_record()
            .returning(|_| Box::pin(async { Ok(1) }));

        let cash_flow_use_case = CashFlowUseCase::new(Arc::new(mock_cash_flow_repository));

        let record_cash_flow_model = RecordCashFlowModel {
            amount: 100.0,
            category: "Food".to_string(),
            description: "Lunch".to_string(),
        };

        let result = cash_flow_use_case.record(record_cash_flow_model).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_with_date_success() {
        let mut mock_cash_flow_repository = MockCashFlowRepository::new();

        mock_cash_flow_repository
            .expect_record()
            .returning(|_| Box::pin(async { Ok(1) }));

        let cash_flow_use_case = CashFlowUseCase::new(Arc::new(mock_cash_flow_repository));

        let record_cash_flow_with_date_model = RecordCashFlowWithDateModel {
            amount: 100.0,
            category: "Food".to_string(),
            description: "Lunch".to_string(),
            date: "2023-10-01".to_string(),
        };

        let result = cash_flow_use_case
            .record_with_date(record_cash_flow_with_date_model)
            .await;

        assert!(result.is_ok());
    }
}
