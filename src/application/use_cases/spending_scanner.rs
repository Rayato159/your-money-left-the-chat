use anyhow::Result;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{
    repositories::spending_scanner::SpendingScannerRepository,
    value_objects::spending_scanner::{Range, SpendingScannerFilter, SpendingScannerModel},
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::spending_scanner::SpendingScannerUseCase,
        domain::{
            entities::my_ledger::MyLedger,
            repositories::spending_scanner::MockSpendingScannerRepository,
            value_objects::spending_scanner::{Range, SpendingScannerFilter},
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
}
