use std::{sync::Arc, time::Instant};

use intent_core::ChainId;
use intent_evm::EvmAdapter;
use intent_db::sea_orm::DatabaseConnection;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub start_time: Instant,
    pub config: Arc<Config>,
    pub evm_adapter: Arc<EvmAdapter>,
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new(config: Arc<Config>) -> Self {
        let evm_adapter = EvmAdapter::new(
            ChainId::BaseSepolia,
            &config.evm.base_sepolia.rpc_url,
            &config.evm.base_sepolia.gateway_address,
            config.evm.base_sepolia.private_key.as_deref(),
        )
        .expect("Failed to create EVM adapter");

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
            
        let db = intent_db::init_db(&database_url)
            .await
            .expect("Failed to initialize database");

        AppState {
            start_time: Instant::now(),
            config,
            evm_adapter: Arc::new(evm_adapter),
            db,
        }
    }
}
