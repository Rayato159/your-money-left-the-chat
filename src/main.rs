use std::{env, sync::Arc};

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
use your_money_left_the_chat::{application::use_cases::cash_flow::CashFlow, infrastructure};

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = env::args().nth(1).expect("flag --db-url is required");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("🦀 Let's roll your money.");

    let db_pool = infrastructure::database::conn(&db_url)?;
    let db_pool_artifact = Arc::new(db_pool);

    let cash_flow_service = CashFlow::new(Arc::clone(&db_pool_artifact))
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    cash_flow_service.waiting().await?;
    Ok(())
}
