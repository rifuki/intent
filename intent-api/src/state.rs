use std::{sync::Arc, time::Instant};

use intent_core::ChainId;
use intent_evm::EvmAdapter;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub start_time: Instant,
    pub config: Arc<Config>,
    pub evm_adapter: Arc<EvmAdapter>,
}

impl AppState {
    pub fn new(config: Arc<Config>) -> Self {
        let evm_adapter = EvmAdapter::new(
            ChainId::BaseSepolia,
            &config.evm.base_sepolia.rpc_url,
            &config.evm.base_sepolia.gateway_address,
            config.evm.base_sepolia.private_key.as_deref(),
        )
        .expect("Failed to create EVM adapter");

        AppState {
            start_time: Instant::now(),
            config,
            evm_adapter: Arc::new(evm_adapter),
        }
    }
}
