use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // let service = SolanaChad::new().serve(stdio()).await.inspect_err(|e| {
    //     tracing::error!("serving error: {:?}", e);
    // })?;

    // service.waiting().await?;
    Ok(())
}
