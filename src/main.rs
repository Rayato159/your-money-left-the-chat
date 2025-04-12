use std::{env, sync::Arc};

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
use your_money_left_the_chat::{
    application::use_cases::{
        cash_flow::CashFlowUseCase, debt_radar::DebtRadarUseCase,
        spending_scanner::SpendingScannerUseCase,
    },
    infrastructure::{
        database::{
            conn,
            repositories::{
                cash_flow::CashFlowSqlite, debt_radar::DebtRadarSqlite,
                spending_scanner::SpendingScannerSqlite,
            },
        },
        mcp_handler::MCPHandler,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let db_path = env::args().nth(1).expect("arg db_path is required");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("🦀 Let's roll your money.");

    let db_pool = conn(&db_path)?;
    let db_pool_artifact = Arc::new(db_pool);

    let cash_flow_use_case = {
        let cash_flow_repository = CashFlowSqlite::new(Arc::clone(&db_pool_artifact));
        CashFlowUseCase::new(Arc::new(cash_flow_repository))
    };

    let spending_scanner_use_case = {
        let spending_scanner_repository = SpendingScannerSqlite::new(Arc::clone(&db_pool_artifact));
        SpendingScannerUseCase::new(Arc::new(spending_scanner_repository))
    };

    let debt_radar_use_case = {
        let debt_radar_repository = DebtRadarSqlite::new(Arc::clone(&db_pool_artifact));
        DebtRadarUseCase::new(Arc::new(debt_radar_repository))
    };

    let service = MCPHandler::new(
        Arc::new(cash_flow_use_case),
        Arc::new(spending_scanner_use_case),
        Arc::new(debt_radar_use_case),
    )
    .serve(stdio())
    .await
    .inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;

    Ok(())
}
