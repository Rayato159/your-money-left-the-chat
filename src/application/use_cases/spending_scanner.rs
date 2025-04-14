use anyhow::Result;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{
    repositories::spending_scanner::SpendingScannerRepository,
    value_objects::spending_scanner::{
        AddMonthlySpendingModel, MonthlySpendingModel, Range, RemoveMonthlySpendingModel,
        SpendingScannerFilter, SpendingScannerModel,
    },
};

#[derive(Clone)]
pub struct SpendingScannerUseCase {
    spending_scanner_repository: Arc<dyn SpendingScannerRepository + Send + Sync + 'static>,
}

impl SpendingScannerUseCase {
    pub fn new(
        spending_scanner_repository: Arc<dyn SpendingScannerRepository + Send + Sync + 'static>,
    ) -> Self {
        Self {
            spending_scanner_repository,
        }
    }

    pub async fn scan(
        &self,
        spending_scanner_filer: SpendingScannerFilter,
    ) -> Result<Vec<SpendingScannerModel>> {
        let results = match spending_scanner_filer.filter {
            Range::Today => self.spending_scanner_repository.today().await?,
            Range::ThisMonth => self.spending_scanner_repository.this_month().await?,
            Range::ThisYear => self.spending_scanner_repository.this_year().await?,
            Range::Lifetime => self.spending_scanner_repository.lifetime().await?,
            Range::Custom { start, end } => {
                self.spending_scanner_repository.custom(start, end).await?
            }
        };

        Ok(results
            .iter()
            .map(|r| r.to_spending_scanner_model())
            .collect::<Vec<SpendingScannerModel>>())
    }

    pub async fn visualize(
        &self,
        spending_scanner_filer: SpendingScannerFilter,
    ) -> Result<HashMap<String, f32>> {
        let results = match spending_scanner_filer.filter {
            Range::Today => self.spending_scanner_repository.today().await?,
            Range::ThisMonth => self.spending_scanner_repository.this_month().await?,
            Range::ThisYear => self.spending_scanner_repository.this_year().await?,
            Range::Lifetime => self.spending_scanner_repository.lifetime().await?,
            Range::Custom { start, end } => {
                self.spending_scanner_repository.custom(start, end).await?
            }
        };

        let map_by_category =
            results
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, r| {
                    let category = r.category.clone();
                    let amount = r.amount;

                    acc.entry(category.clone()).or_insert(0.0);
                    *acc.get_mut(&category).unwrap() += amount;

                    acc
                });

        Ok(map_by_category)
    }

    pub async fn view_all_monthly_spending_list(&self) -> Result<Vec<MonthlySpendingModel>> {
        let entities = self
            .spending_scanner_repository
            .view_all_monthly_spending()
            .await?;

        let results = entities
            .iter()
            .map(|r| r.to_model())
            .collect::<Vec<MonthlySpendingModel>>();

        Ok(results)
    }

    pub async fn add_monthly_spending(
        &self,
        add_monthly_spending_model: AddMonthlySpendingModel,
    ) -> Result<i32> {
        self.spending_scanner_repository
            .add_monthly_spending(add_monthly_spending_model.to_dto())
            .await
    }

    pub async fn remove_monthly_spending(
        &self,
        remove_monthly_spending_model: RemoveMonthlySpendingModel,
    ) -> Result<()> {
        self.spending_scanner_repository
            .remove_monthly_spending(remove_monthly_spending_model.id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::spending_scanner::SpendingScannerUseCase,
        domain::{
            entities::{monthly_spending::MonthlySpending, my_ledger::MyLedger},
            repositories::spending_scanner::MockSpendingScannerRepository,
            value_objects::spending_scanner::{
                AddMonthlySpendingModel, Range, RemoveMonthlySpendingModel, SpendingScannerFilter,
            },
        },
    };

    #[tokio::test]
    async fn test_scan_success() {
        let mut mock_spending_scanner_repository = MockSpendingScannerRepository::new();

        mock_spending_scanner_repository
            .expect_today()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![
                        MyLedger {
                            id: 1,
                            amount: 100.0,
                            category: "Food".to_string(),
                            date: "2023-10-01".to_string(),
                            description: "Lunch".to_string(),
                        },
                        MyLedger {
                            id: 2,
                            amount: 200.0,
                            category: "Food".to_string(),
                            date: "2023-10-01".to_string(),
                            description: "Lunch".to_string(),
                        },
                        MyLedger {
                            id: 3,
                            amount: 150.0,
                            category: "Food".to_string(),
                            date: "2023-10-01".to_string(),
                            description: "Coffee".to_string(),
                        },
                    ])
                })
            });

        let spending_scanner_use_case =
            SpendingScannerUseCase::new(Arc::new(mock_spending_scanner_repository));

        let spending_scanner_filter = SpendingScannerFilter {
            filter: Range::Today,
        };

        let result = spending_scanner_use_case
            .scan(spending_scanner_filter)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_visualize_success() {
        let mut mock_spending_scanner_repository = MockSpendingScannerRepository::new();

        mock_spending_scanner_repository
            .expect_today()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![
                        MyLedger {
                            id: 1,
                            amount: 100.0,
                            category: "Food".to_string(),
                            date: "2023-10-01".to_string(),
                            description: "Lunch".to_string(),
                        },
                        MyLedger {
                            id: 2,
                            amount: 200.0,
                            category: "Food".to_string(),
                            date: "2023-10-01".to_string(),
                            description: "Lunch".to_string(),
                        },
                        MyLedger {
                            id: 3,
                            amount: 150.0,
                            category: "Coffee".to_string(),
                            date: "2023-10-01".to_string(),
                            description: "Coffee".to_string(),
                        },
                    ])
                })
            });

        let spending_scanner_use_case =
            SpendingScannerUseCase::new(Arc::new(mock_spending_scanner_repository));

        let spending_scanner_filter = SpendingScannerFilter {
            filter: Range::Today,
        };

        let result = spending_scanner_use_case
            .visualize(spending_scanner_filter)
            .await;

        assert!(result.is_ok());

        let lunch = result.as_ref().unwrap().get("Food").unwrap();
        let coffee = result.as_ref().unwrap().get("Coffee").unwrap();

        assert_eq!(*lunch, 300.0);
        assert_eq!(*coffee, 150.0);
    }

    #[tokio::test]
    async fn test_view_all_monthly_spending_success() {
        let mut mock_spending_scanner_repository = MockSpendingScannerRepository::new();

        mock_spending_scanner_repository
            .expect_view_all_monthly_spending()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![
                        MonthlySpending {
                            id: 1,
                            title: "Test 1".to_string(),
                            amount: 100.0,
                            due_date: "2023-10-01".to_string(),
                        },
                        MonthlySpending {
                            id: 2,
                            title: "Test 2".to_string(),
                            amount: 200.0,
                            due_date: "2023-10-01".to_string(),
                        },
                    ])
                })
            });

        let spending_scanner_use_case =
            SpendingScannerUseCase::new(Arc::new(mock_spending_scanner_repository));

        let result = spending_scanner_use_case
            .view_all_monthly_spending_list()
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_add_monthly_spending_success() {
        let mut mock_spending_scanner_repository = MockSpendingScannerRepository::new();

        mock_spending_scanner_repository
            .expect_add_monthly_spending()
            .returning(|_| Box::pin(async { Ok(1) }));

        let spending_scanner_use_case =
            SpendingScannerUseCase::new(Arc::new(mock_spending_scanner_repository));

        let result = spending_scanner_use_case
            .add_monthly_spending(AddMonthlySpendingModel {
                title: "Test".to_string(),
                amount: 100.0,
                due_date: "2023-10-01".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_monthly_spending_success() {
        let mut mock_spending_scanner_repository = MockSpendingScannerRepository::new();

        mock_spending_scanner_repository
            .expect_remove_monthly_spending()
            .returning(|_| Box::pin(async { Ok(()) }));

        let spending_scanner_use_case =
            SpendingScannerUseCase::new(Arc::new(mock_spending_scanner_repository));

        let result = spending_scanner_use_case
            .remove_monthly_spending(RemoveMonthlySpendingModel { id: 1 })
            .await;

        assert!(result.is_ok());
    }
}
