use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainType {
    Evm,
    Solana,
    Sui,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainId {
    // EVM
    ArbitrumSepolia,
    BaseSepolia,
    // Solana
    SolanaDevnet,
    SolanaMainnet,
    // Sui
    SuiTestnet,
    SuiMainnet,
}

impl ChainId {
    pub fn chain_type(&self) -> ChainType {
        match self {
            Self::ArbitrumSepolia | Self::BaseSepolia => ChainType::Evm,
            Self::SolanaDevnet | Self::SolanaMainnet => ChainType::Solana,
            Self::SuiTestnet | Self::SuiMainnet => ChainType::Sui,
        }
    }

    pub fn evm_chain_id(&self) -> Option<u64> {
        match self {
            Self::BaseSepolia => Some(84532),
            Self::ArbitrumSepolia => Some(421614),
            _ => None,
        }
    }

    pub fn default_rpc(&self) -> &'static str {
        match self {
            Self::ArbitrumSepolia => "https://sepolia-rollup.arbitrum.io/rpc",
            Self::BaseSepolia => "https://sepolia.base.org",
            Self::SolanaDevnet => "https://api.devnet.solana.com",
            Self::SolanaMainnet => "https://api.mainnet-beta.solana.com",
            Self::SuiTestnet => "https://fullnode.testnet.sui.io:443",
            Self::SuiMainnet => "https://fullnode.mainnet.sui.io:443",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentStatus {
    Pending,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intent {
    pub id: Option<String>,
    pub chain: ChainId,
    pub creator: String,
    pub input_token: TokenInfo,
    pub input_amount: String,
    pub output_token: TokenInfo,
    pub min_output_amount: String,
    pub deadline: u64,
    pub status: IntentStatus,
}

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("Intent not found: {0}")]
    NotFound(String),
    #[error("Transaction failed: {0}")]
    TxFailed(String),
}

#[async_trait]
pub trait ChainAdapter: Send + Sync {
    fn chain_id(&self) -> ChainId;
    async fn submit_intent(&self, intent: &Intent) -> Result<String, AdapterError>;
    async fn get_intent(&self, intent_id: &str) -> Result<Intent, AdapterError>;
    async fn cancel_intent(&self, intent_id: &str) -> Result<String, AdapterError>;
}
