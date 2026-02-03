use intent_core::{ChainId, TokenInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateIntentRequest {
    pub chain: ChainId,
    pub creator: String,
    pub input_token: TokenInfo,
    pub input_amount: String,
    pub output_token: TokenInfo,
    pub min_output_amount: String,
    pub deadline: u64,
}

#[derive(Debug, Serialize)]
pub struct TxHashResponse {
    pub tx_hash: String,
}

#[derive(Debug, Serialize)]
pub struct ChainsResponse {
    pub chains: Vec<ChainInfo>,
}

#[derive(Debug, Serialize)]
pub struct ChainInfo {
    pub id: ChainId,
    pub name: String,
    pub chain_type: String,
}
