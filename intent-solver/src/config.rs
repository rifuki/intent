use dotenv::dotenv;
use std::env;

pub struct SolverConfig {
    pub chain_id: String,
    pub rpc_url: String,
    pub gateway_address: String,
    pub private_key: String,
    pub min_profit_bps: u64, // Basis points (100 = 1%)
    pub poll_interval_secs: u64,
}

impl SolverConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            chain_id: env::var("CHAIN_ID").unwrap_or_else(|_| "base_sepolia".to_string()),
            rpc_url: env::var("RPC_URL").expect("RPC_URL must be set"),
            gateway_address: env::var("GATEWAY_ADDRESS").expect("GATEWAY_ADDRESS must be set"),
            private_key: env::var("SOLVER_PRIVATE_KEY").expect("SOLVER_PRIVATE_KEY must be set"),
            min_profit_bps: env::var("MIN_PROFIT_BPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50), // Default 0.5%
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
        }
    }
}
