use std::{env, sync::Arc};

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
use your_money_left_the_chat::{
    application::use_cases::cash_flow::CashFlowUseCase,
    infrastructure::{
        self, database::repositories::cash_flow::CashFlowSQLite,
        mcp_handler::cash_flow::CashFlowMCPHandler,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let db_path = env::args().nth(1).expect("flag db path is required");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("🦀 Let's roll your money.");

    let db_pool = infrastructure::database::conn(&db_path)?;
    let db_pool_artifact = Arc::new(db_pool);

    let cash_flow_use_case = {
        let cash_flow_repository = CashFlowSQLite::new(Arc::clone(&db_pool_artifact)).await;
        CashFlowUseCase::new(Arc::new(cash_flow_repository))
    };

    let cash_flow_service = CashFlowMCPHandler::new(Arc::new(cash_flow_use_case))
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    cash_flow_service.waiting().await?;
    Ok(())
}
