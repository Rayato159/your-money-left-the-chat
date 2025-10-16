use std::sync::Arc;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
use your_money_left_the_chat::{
    application::use_cases::{
        cash_flow::CashFlowUseCase, spending_scanner::SpendingScannerUseCase,
        tax_simulator::TaxSimulatorUseCase,
    },
    config,
    infrastructure::{
        database::{
            conn,
            repositories::{
                cash_flow::CashFlowSqlite, spending_scanner::SpendingScannerSqlite,
                tax_simulator::TaxSimulatorSqlite,
            },
        },
        mcp_handler::MCPHandler,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("🦀 Let's roll your money.");

    let db_pool = conn(&config.database_url)?;
    let db_pool_artifact = Arc::new(db_pool);

    let cash_flow_use_case = {
        let cash_flow_repository = CashFlowSqlite::new(Arc::clone(&db_pool_artifact));
        CashFlowUseCase::new(Arc::new(cash_flow_repository))
    };

    let spending_scanner_use_case = {
        let spending_scanner_repository = SpendingScannerSqlite::new(Arc::clone(&db_pool_artifact));
        SpendingScannerUseCase::new(Arc::new(spending_scanner_repository))
    };

    let tax_simulator_use_case = {
        let tax_simulator_repository = TaxSimulatorSqlite::new(Arc::clone(&db_pool_artifact));
        TaxSimulatorUseCase::new(Arc::new(tax_simulator_repository))
    };

    let service = MCPHandler::new(
        Arc::new(cash_flow_use_case),
        Arc::new(spending_scanner_use_case),
        Arc::new(tax_simulator_use_case),
    )
    .serve(stdio())
    .await
    .inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;

    Ok(())
}
