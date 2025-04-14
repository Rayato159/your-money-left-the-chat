use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::tax_simulator::TaxSimulatorRepository,
    value_objects::tax_simulator::{
        AddTaxDeductionsListModel, RemoveTaxDeductionsListModel, TaxDeductionsListModel,
        TaxSimulateRequestModel, TaxSimulateResult,
    },
};

/// Reference: https://www.rd.go.th/59670.html
/// (Lower, Upper, TaxRate, Constraint)
const TAX_RANGE: &[(f32, f32, f32, f32)] = &[
    (0.0, 150_000.0, 0.0, 0.0),
    (150_000.0, 300_000.0, 0.05, 0.0),
    (300_000.0, 500_000.0, 0.1, 27_500.0),
    (500_000.0, 750_000.0, 0.15, 27_500.0),
    (750_000.0, 1_000_000.0, 0.2, 65_000.0),
    (1_000_000.0, 2_000_000.0, 0.25, 115_000.0),
    (2_000_000.0, 5_000_000.0, 0.3, 365_000.0),
    (5_000_000.0, f32::MAX, 0.35, 1_265_000.0),
];

#[derive(Clone)]
pub struct TaxSimulatorUseCase {
    tax_simulator_repository: Arc<dyn TaxSimulatorRepository + Send + Sync + 'static>,
}

impl TaxSimulatorUseCase {
    pub fn new(
        tax_simulator_repository: Arc<dyn TaxSimulatorRepository + Send + Sync + 'static>,
    ) -> Self {
        Self {
            tax_simulator_repository,
        }
    }

    pub async fn view_all_tax_deductions_list(&self) -> Result<Vec<TaxDeductionsListModel>> {
        let entities = self
            .tax_simulator_repository
            .view_all_tax_deductions_list()
            .await?;

        let results = entities
            .iter()
            .map(|e| e.to_model())
            .collect::<Vec<TaxDeductionsListModel>>();

        Ok(results)
    }

    pub async fn add_tax_deduction_list(
        &self,
        add_tax_deduction_list_model: AddTaxDeductionsListModel,
    ) -> Result<i32> {
        let result_id = self
            .tax_simulator_repository
            .add_tax_deduction_list(add_tax_deduction_list_model.to_dto())
            .await?;

        Ok(result_id)
    }

    pub async fn remove_tax_deduction_list(
        &self,
        remove_tax_deduction_list_model: RemoveTaxDeductionsListModel,
    ) -> Result<()> {
        self.tax_simulator_repository
            .remove_tax_deduction_list(remove_tax_deduction_list_model.id)
            .await?;

        Ok(())
    }

    pub async fn simulate(
        &self,
        tax_simulate_request_model: TaxSimulateRequestModel,
    ) -> Result<TaxSimulateResult> {
        let incomes = self
            .tax_simulator_repository
            .view_all_income_by_year(tax_simulate_request_model.year)
            .await?;

        let tax_deductions_list = self
            .tax_simulator_repository
            .view_all_tax_deductions_list()
            .await?;

        let mut tax = TaxSimulateResult { must_pay: 0.0 };

        let total_income = incomes.iter().fold(0.0, |acc, income| acc + income.amount);

        // Calculate original tax
        for r in TAX_RANGE.iter() {
            if r.0 > total_income || total_income <= r.1 {
                tax.must_pay = ((total_income - r.0) * r.2) + r.3;
                break;
            }
        }

        // Deduction calculate
        for td in tax_deductions_list.iter() {
            tax.must_pay -= td.amount;
        }

        match tax.must_pay <= 0.0 {
            true => {
                tax.must_pay = 0.0;
                Ok(tax)
            }
            _ => Ok(tax),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::tax_simulator::TaxSimulatorUseCase,
        domain::{
            entities::{my_ledger::MyLedger, tax_deductions_list::TaxDeductionsList},
            repositories::tax_simulator::MockTaxSimulatorRepository,
            value_objects::tax_simulator::{
                AddTaxDeductionsListModel, RemoveTaxDeductionsListModel,
            },
        },
    };

    #[tokio::test]
    async fn test_view_all_tax_deductions_list_success() {
        let mut mock_tax_simulator_repository = MockTaxSimulatorRepository::new();

        mock_tax_simulator_repository
            .expect_view_all_tax_deductions_list()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![
                        TaxDeductionsList {
                            id: 1,
                            amount: 1000.0,
                            title: "Test".to_string(),
                        },
                        TaxDeductionsList {
                            id: 2,
                            amount: 2000.0,
                            title: "Test2".to_string(),
                        },
                    ])
                })
            });

        let tax_simulator_use_case =
            TaxSimulatorUseCase::new(Arc::new(mock_tax_simulator_repository));

        let result = tax_simulator_use_case.view_all_tax_deductions_list().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_add_tax_deduction_list_success() {
        let mut mock_tax_simulator_repository = MockTaxSimulatorRepository::new();

        mock_tax_simulator_repository
            .expect_add_tax_deduction_list()
            .returning(|_| Box::pin(async { Ok(1) }));

        let add_tax_deduction_list_model = AddTaxDeductionsListModel {
            amount: 1000.0,
            title: "Test".to_string(),
        };

        let tax_simulator_use_case =
            TaxSimulatorUseCase::new(Arc::new(mock_tax_simulator_repository));

        let result = tax_simulator_use_case
            .add_tax_deduction_list(add_tax_deduction_list_model)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_tax_deduction_list_success() {
        let mut mock_tax_simulator_repository = MockTaxSimulatorRepository::new();

        mock_tax_simulator_repository
            .expect_remove_tax_deduction_list()
            .returning(|_| Box::pin(async { Ok(()) }));

        let tax_simulator_use_case =
            TaxSimulatorUseCase::new(Arc::new(mock_tax_simulator_repository));

        let result = tax_simulator_use_case
            .remove_tax_deduction_list(RemoveTaxDeductionsListModel { id: 1 })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_simulate_success() {
        let mut mock_tax_simulator_repository = MockTaxSimulatorRepository::new();

        mock_tax_simulator_repository
            .expect_view_all_income_by_year()
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        MyLedger {
                            id: 1,
                            amount: 250_000.0,
                            category: "INCOME".to_string(),
                            date: "2025-01-01".to_string(),
                            description: "Monthly Salary".to_string(),
                        },
                        MyLedger {
                            id: 2,
                            amount: 250_000.0,
                            category: "SALARY".to_string(),
                            date: "2025-02-01".to_string(),
                            description: "Monthly Salary".to_string(),
                        },
                        MyLedger {
                            id: 3,
                            amount: 250_000.0,
                            category: "INCOME".to_string(),
                            date: "2025-03-01".to_string(),
                            description: "Monthly Salary".to_string(),
                        },
                        MyLedger {
                            id: 4,
                            amount: 250_000.0,
                            category: "INCOME".to_string(),
                            date: "2025-04-01".to_string(),
                            description: "Monthly Salary".to_string(),
                        },
                    ])
                })
            });

        mock_tax_simulator_repository
            .expect_view_all_tax_deductions_list()
            .returning(|| {
                Box::pin(async {
                    Ok(vec![
                        TaxDeductionsList {
                            id: 1,
                            amount: 60_000.0,
                            title: "Personal".to_string(),
                        },
                        TaxDeductionsList {
                            id: 2,
                            amount: 9000.0,
                            title: "Social Security".to_string(),
                        },
                    ])
                })
            });

        let expected = 46000.0;

        let tax_simulator_use_case =
            TaxSimulatorUseCase::new(Arc::new(mock_tax_simulator_repository));

        let tax_simulate_request_model =
            crate::domain::value_objects::tax_simulator::TaxSimulateRequestModel { year: 2025 };

        let result = tax_simulator_use_case
            .simulate(tax_simulate_request_model)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().must_pay, expected);
    }
}
