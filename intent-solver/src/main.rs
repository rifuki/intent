mod config;
mod executor;
mod monitor;
mod strategy;

use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use config::SolverConfig;
use executor::IntentExecutor;
use monitor::IntentMonitor;
use strategy::ProfitabilityChecker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .pretty()
        .init();

    info!("🤖 Intent Solver starting...");

    // Load config
    let config = SolverConfig::from_env();
    info!("📋 Config loaded: chain={}", config.chain_id);

    // Initialize components
    let strategy = Arc::new(ProfitabilityChecker::new(config.min_profit_bps));
    let executor = Arc::new(IntentExecutor::new(
        &config.rpc_url,
        &config.gateway_address,
        &config.private_key,
    )?);

    // Create and run monitor
    let monitor = IntentMonitor::new(
        &config.rpc_url,
        &config.gateway_address,
        strategy,
        executor,
    )?;

    info!("👀 Starting intent monitor...");
    monitor.run().await?;

    Ok(())
}
