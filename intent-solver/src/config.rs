use dotenv::dotenv;
use std::env;

pub struct SolverConfig {
    pub chain_id: String,
    pub chain_id_numeric: u64,
    pub rpc_url: String,
    pub gateway_address: String,
    pub private_key: String,
    pub min_profit_bps: u64,
    pub poll_interval_secs: u64,
    pub zerox_api_key: Option<String>,
    pub database_url: String,
}

impl SolverConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let chain_id = env::var("CHAIN_ID").unwrap_or_else(|_| "base_sepolia".to_string());
        
        let chain_id_numeric = match chain_id.as_str() {
            "base_mainnet" => 8453,
            "base_sepolia" => 84532,
            "arbitrum" => 42161,
            "arbitrum_sepolia" => 421614,
            "ethereum" => 1,
            _ => 84532,
        };

        Self {
            chain_id,
            chain_id_numeric,
            rpc_url: env::var("RPC_URL").expect("RPC_URL must be set"),
            gateway_address: env::var("GATEWAY_ADDRESS").expect("GATEWAY_ADDRESS must be set"),
            private_key: env::var("SOLVER_PRIVATE_KEY").expect("SOLVER_PRIVATE_KEY must be set"),
            min_profit_bps: env::var("MIN_PROFIT_BPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50),
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            zerox_api_key: env::var("ZEROX_API_KEY").ok(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
