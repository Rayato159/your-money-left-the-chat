use std::sync::Arc;

use crate::domain::{
    repositories::spending_scanner::SpendingScannerRepository,
    value_objects::spending_scanner::{Range, SpendingScannerFilter, SpendingScannerModel},
};

#[derive(Clone)]
pub struct SpendingScannerUseCase {
    pub spending_scanner_repository: Arc<dyn SpendingScannerRepository + Send + Sync + 'static>,
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
    ) -> anyhow::Result<Vec<SpendingScannerModel>> {
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
}
