use anyhow::Result;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{
    entities::debt_ledger::DebtLedger,
    repositories::debt_radar::DebtRadarRepository,
    value_objects::debt_radar::{DebtViewModel, PaidDebtModel, RecordDebtModel},
};

#[derive(Clone)]
pub struct DebtRadarUseCase {
    debt_radar_repository: Arc<dyn DebtRadarRepository + Send + Sync + 'static>,
}

impl DebtRadarUseCase {
    pub fn new(
        debt_radar_repository: Arc<dyn DebtRadarRepository + Send + Sync + 'static>,
    ) -> Self {
        Self {
            debt_radar_repository,
        }
    }

    pub async fn record(&self, record_debt_radar_model: RecordDebtModel) -> Result<i32> {
        self.debt_radar_repository
            .record(record_debt_radar_model.to_dto())
            .await
    }

    pub async fn view_all(&self) -> Result<Vec<DebtViewModel>> {
        let debt_entities = self.debt_radar_repository.view_all().await?;

        let mut debt_maps: HashMap<&str, DebtLedger> = HashMap::new();

        for de in debt_entities.iter() {
            let who = de.who.as_str();

            if let Some(amount) = debt_maps.get_mut(who) {
                amount.amount += de.amount;
            } else {
                debt_maps.insert(who, de.clone());
            }
        }

        let debt_view_models = debt_maps
            .values()
            .to_owned()
            .into_iter()
            .map(|d| d.to_model())
            .collect::<Vec<DebtViewModel>>();

        Ok(debt_view_models)
    }

    pub async fn view_by_that_bro(&self, who: &str) -> Result<Option<DebtViewModel>> {
        let debt_entity = match self.debt_radar_repository.view_by_that_bro(who).await {
            Ok(debt_entity) => debt_entity,
            Err(_) => return Ok(None),
        };

        let total_debt = debt_entity.iter().fold(0.0, |acc, d| acc + d.amount);

        let debt_view_model = DebtViewModel {
            amount: total_debt,
            who: who.to_string(),
        };

        Ok(Some(debt_view_model))
    }

    pub async fn record_paid_debt(&self, paid_debt_model: PaidDebtModel) -> Result<i32> {
        self.debt_radar_repository
            .record_paid_debt(paid_debt_model.to_dto())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::debt_radar::MockDebtRadarRepository;

    #[tokio::test]
    async fn test_record_success() {
        let mut mock_debt_radar_repository = MockDebtRadarRepository::new();

        mock_debt_radar_repository
            .expect_record()
            .returning(|_| Box::pin(async { Ok(1) }));

        let record_debt_model = RecordDebtModel {
            amount: 100.0,
            category: "Test".to_string(),
            description: "Test description".to_string(),
            who: "Test User".to_string(),
        };

        let debt_radar_use_case = DebtRadarUseCase::new(Arc::new(mock_debt_radar_repository));

        let result = debt_radar_use_case.record(record_debt_model).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_view_all_success() {
        let mut mock_debt_radar_repository = MockDebtRadarRepository::new();

        mock_debt_radar_repository.expect_view_all().returning(|| {
            Box::pin(async {
                Ok(vec![
                    DebtLedger {
                        id: 1,
                        amount: 100.0,
                        category: "Test".to_string(),
                        description: "Test description".to_string(),
                        who: "Test User".to_string(),
                        date: "2023-10-01".to_string(),
                    },
                    DebtLedger {
                        id: 2,
                        amount: 200.0,
                        category: "Test".to_string(),
                        description: "Test description".to_string(),
                        who: "Test User".to_string(),
                        date: "2023-10-02".to_string(),
                    },
                ])
            })
        });

        let debt_radar_use_case = DebtRadarUseCase::new(Arc::new(mock_debt_radar_repository));

        let result = debt_radar_use_case.view_all().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(0).unwrap().amount, 300.0);
    }

    #[tokio::test]
    async fn test_view_by_that_bro_success() {
        let mut mock_debt_radar_repository = MockDebtRadarRepository::new();

        mock_debt_radar_repository
            .expect_view_by_that_bro()
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        DebtLedger {
                            id: 1,
                            amount: 100.0,
                            category: "Test".to_string(),
                            description: "Test description".to_string(),
                            who: "Test User".to_string(),
                            date: "2023-10-01".to_string(),
                        },
                        DebtLedger {
                            id: 1,
                            amount: 400.0,
                            category: "Test".to_string(),
                            description: "Test description".to_string(),
                            who: "Test User".to_string(),
                            date: "2023-10-01".to_string(),
                        },
                    ])
                })
            });

        let debt_radar_use_case = DebtRadarUseCase::new(Arc::new(mock_debt_radar_repository));

        let result = debt_radar_use_case.view_by_that_bro("Test User").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().amount, 500.0);
    }

    #[tokio::test]
    async fn test_record_paid_debt_success() {
        let mut mock_debt_radar_repository = MockDebtRadarRepository::new();

        mock_debt_radar_repository
            .expect_record_paid_debt()
            .returning(|_| Box::pin(async { Ok(1) }));

        let paid_debt_model = PaidDebtModel {
            amount: 100.0,
            who: "Test User".to_string(),
            date: "2023-10-01".to_string(),
        };

        let debt_radar_use_case = DebtRadarUseCase::new(Arc::new(mock_debt_radar_repository));

        let result = debt_radar_use_case.record_paid_debt(paid_debt_model).await;

        assert!(result.is_ok());
    }
}
