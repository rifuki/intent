use alloy::primitives::U256;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

/// 0x Swap API price response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZeroExPriceResponse {
    pub buy_amount: String,
    pub sell_amount: String,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
}

/// 0x API Error response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ZeroExError {
    pub reason: Option<String>,
    pub code: Option<i32>,
}

/// DEX Aggregator for getting real market prices
pub struct DexAggregator {
    client: Client,
    api_key: Option<String>,
    chain_id: u64,
}

impl DexAggregator {
    /// Create new DEX aggregator for a specific chain
    /// Base Mainnet = 8453, Base Sepolia = 84532
    pub fn new(chain_id: u64, api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            chain_id,
        }
    }

    /// Get the 0x API base URL for the chain
    fn get_base_url(&self) -> &'static str {
        match self.chain_id {
            8453 => "https://api.0x.org",      // Base Mainnet
            84532 => "https://api.0x.org",     // Base Sepolia (same endpoint, different chainId param)
            1 => "https://api.0x.org",         // Ethereum
            42161 => "https://api.0x.org",     // Arbitrum
            _ => "https://api.0x.org",
        }
    }

    /// Get real market quote from 0x API
    /// Returns the output amount for given input
    pub async fn get_quote(
        &self,
        sell_token: &str,
        buy_token: &str,
        sell_amount: U256,
        taker_address: &str,
    ) -> Result<U256, String> {
        let base_url = self.get_base_url();
        let url = format!(
            "{}/swap/v1/price?chainId={}&sellToken={}&buyToken={}&sellAmount={}&takerAddress={}",
            base_url,
            self.chain_id,
            sell_token,
            buy_token,
            sell_amount,
            taker_address
        );

        info!(
            "🔍 Fetching quote from 0x: {} -> {} (amount: {})",
            sell_token, buy_token, sell_amount
        );

        let mut request = self.client.get(&url);
        
        // Add API key header if available
        if let Some(ref key) = self.api_key {
            request = request.header("0x-api-key", key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("⚠️ 0x API error: {} - {}", status, error_text);
            return Err(format!("0x API error: {} - {}", status, error_text));
        }

        let price_response: ZeroExPriceResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let buy_amount: U256 = price_response
            .buy_amount
            .parse()
            .map_err(|e| format!("Failed to parse buy_amount: {}", e))?;

        info!(
            "✅ Got quote: sell {} -> buy {} (gas: {:?})",
            price_response.sell_amount,
            price_response.buy_amount,
            price_response.gas
        );

        Ok(buy_amount)
    }

    /// Get quote with fallback to mock if API fails (useful for testnet)
    pub async fn get_quote_with_fallback(
        &self,
        sell_token: &str,
        buy_token: &str,
        sell_amount: U256,
        taker_address: &str,
    ) -> U256 {
        match self.get_quote(sell_token, buy_token, sell_amount, taker_address).await {
            Ok(amount) => amount,
            Err(e) => {
                warn!("⚠️ 0x API failed, using mock rate: {}", e);
                
                // Hacky fallback for demo: handle USDC (6 decimals) -> WETH (18 decimals)
                // If SELL token is USDC (checking if address ends with... or just assume scale)
                // Real production logic would fetch decimals on-chain.
                
                // For this demo: if sell_amount is small (likely USDC 6 decimals) and we want WETH
                // We scale up by 10^12 (18-6)
                // 1 USDC (10^6) -> ~0.0003 ETH (3 * 10^14)
                
                if sell_token.to_lowercase() == "0x036cbd53842c5426634e7929541ec2318f3dcf7e" 
                   && buy_token.to_lowercase() == "0x4200000000000000000000000000000000000006" {
                    // USDC -> WETH
                    // Rate: 1 USDC = 0.0003 ETH (approx for testnet)
                    // limit profit to be slightly more than min_output if possible for demo
                    // But here let's just use a fixed rate: 1 USDC = 0.0003 ETH
                    
                    let scale = U256::from(10).pow(U256::from(12)); // 18 - 6
                    let eth_amount = sell_amount * scale / U256::from(3000); // 1 ETH = 3000 USDC approx
                    
                    // Add small profit margin for solver (98% of market rate)
                     eth_amount * U256::from(98) / U256::from(100)
                } else {
                    // Default 1:1 for other pairs
                    sell_amount * U256::from(98) / U256::from(100)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dex_aggregator_creation() {
        let dex = DexAggregator::new(8453, None);
        assert_eq!(dex.chain_id, 8453);
    }
}
