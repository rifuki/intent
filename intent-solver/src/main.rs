mod config;
mod dex;
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
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .pretty()
        .init();

    info!("🤖 Intent Solver starting...");

    let config = SolverConfig::from_env();
    info!(
        "📋 Config: chain={} ({}), 0x_api={}",
        config.chain_id,
        config.chain_id_numeric,
        if config.zerox_api_key.is_some() { "✅" } else { "❌" }
    );

    let executor = Arc::new(IntentExecutor::new(
        &config.rpc_url,
        &config.gateway_address,
        &config.private_key,
    )?);
    
    let solver_address = format!("{:?}", executor.solver_address());
    info!("💼 Solver: {}", solver_address);

    let strategy = Arc::new(ProfitabilityChecker::new(
        config.min_profit_bps,
        config.chain_id_numeric,
        config.zerox_api_key,
        solver_address,
    ));

    info!("🔌 connecting to database...");
    let db = intent_db::init_db(&config.database_url).await?;
    info!("✅ database connected");

    let monitor = IntentMonitor::new(
        &config.rpc_url,
        &config.gateway_address,
        strategy,
        executor,
        db,
    )?;

    info!("👀 Monitoring intents with real 0x DEX quotes...");
    monitor.run().await?;

    Ok(())
}
