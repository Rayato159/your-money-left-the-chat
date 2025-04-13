use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::bitcoin_flow::BitcoinFlowRepository,
    value_objects::bitcoin_flow::{BuyBitcoinModel, SellBitcoinModel, ViewBitcoinLedgerModel},
};

#[derive(Clone)]
pub struct BitcoinFlowUseCase {
    bitcoin_flow_repository: Arc<dyn BitcoinFlowRepository + Send + Sync + 'static>,
}

impl BitcoinFlowUseCase {
    pub fn new(
        bitcoin_flow_repository: Arc<dyn BitcoinFlowRepository + Send + Sync + 'static>,
    ) -> Self {
        Self {
            bitcoin_flow_repository,
        }
    }

    pub async fn record_buy(&self, buy_bitcoin_model: BuyBitcoinModel) -> Result<i32> {
        self.bitcoin_flow_repository
            .record_buy(buy_bitcoin_model.to_dto())
            .await
    }

    pub async fn record_sell(&self, sell_bitcoin_model: SellBitcoinModel) -> Result<i32> {
        self.bitcoin_flow_repository
            .record_sell(sell_bitcoin_model.to_dto())
            .await
    }

    pub async fn view_all_buy(&self) -> Result<Vec<ViewBitcoinLedgerModel>> {
        let entities = self.bitcoin_flow_repository.view_all_buy().await?;

        let results = entities
            .into_iter()
            .map(|e| e.to_model())
            .collect::<Vec<ViewBitcoinLedgerModel>>();

        Ok(results)
    }

    pub async fn view_all_sell(&self) -> Result<Vec<ViewBitcoinLedgerModel>> {
        let entities = self.bitcoin_flow_repository.view_all_sell().await?;

        let results = entities
            .into_iter()
            .map(|e| e.to_model())
            .collect::<Vec<ViewBitcoinLedgerModel>>();

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::bitcoin_flow::BitcoinFlowUseCase,
        domain::{
            entities::bitcoin_ledger::{BitcoinBuyLedger, BitcoinSellLedger},
            repositories::bitcoin_flow::MockBitcoinFlowRepository,
            value_objects::bitcoin_flow::{BuyBitcoinModel, SellBitcoinModel},
        },
    };

    #[tokio::test]
    async fn test_record_buy_success() {
        let mut mock_bitcoin_flow_repository = MockBitcoinFlowRepository::new();

        mock_bitcoin_flow_repository
            .expect_record_buy()
            .returning(|_| Box::pin(async { Ok(1) }));

        let bitcoin_flow_use_case = BitcoinFlowUseCase::new(Arc::new(mock_bitcoin_flow_repository));

        let buy_bitcoin_model = BuyBitcoinModel {
            cost: 5000.0,
            amount: 0.1,
            price: 80000.0,
            date: "2023-10-01".to_string(),
        };

        let result = bitcoin_flow_use_case.record_buy(buy_bitcoin_model).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_sell_success() {
        let mut mock_bitcoin_flow_repository = MockBitcoinFlowRepository::new();

        mock_bitcoin_flow_repository
            .expect_record_sell()
            .returning(|_| Box::pin(async { Ok(1) }));

        let bitcoin_flow_use_case = BitcoinFlowUseCase::new(Arc::new(mock_bitcoin_flow_repository));

        let sell_bitcoin_model = SellBitcoinModel {
            cost: 5000.0,
            amount: 0.1,
            price: 80000.0,
            date: "2023-10-01".to_string(),
        };

        let result = bitcoin_flow_use_case.record_sell(sell_bitcoin_model).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_view_all_buy_success() {
        let mut mock_bitcoin_flow_repository = MockBitcoinFlowRepository::new();

        mock_bitcoin_flow_repository
            .expect_view_all_buy()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![BitcoinBuyLedger {
                        id: 1,
                        cost: 5000.0,
                        amount: 0.1,
                        price: 80000.0,
                        date: "2023-10-01".to_string(),
                    }])
                })
            });

        let bitcoin_flow_use_case = BitcoinFlowUseCase::new(Arc::new(mock_bitcoin_flow_repository));

        let result = bitcoin_flow_use_case.view_all_buy().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_view_all_sell_success() {
        let mut mock_bitcoin_flow_repository = MockBitcoinFlowRepository::new();

        mock_bitcoin_flow_repository
            .expect_view_all_sell()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![BitcoinSellLedger {
                        id: 1,
                        cost: 5000.0,
                        amount: 0.1,
                        price: 80000.0,
                        date: "2023-10-01".to_string(),
                    }])
                })
            });

        let bitcoin_flow_use_case = BitcoinFlowUseCase::new(Arc::new(mock_bitcoin_flow_repository));

        let result = bitcoin_flow_use_case.view_all_sell().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
