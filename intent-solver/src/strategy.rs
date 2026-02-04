use alloy::primitives::U256;
use tracing::info;

use crate::dex::DexAggregator;

/// Profitability checker for deciding whether to fill an intent
pub struct ProfitabilityChecker {
    min_profit_bps: u64,
    dex: DexAggregator,
    solver_address: String,
}

impl ProfitabilityChecker {
    pub fn new(min_profit_bps: u64, chain_id: u64, api_key: Option<String>, solver_address: String) -> Self {
        Self {
            min_profit_bps,
            dex: DexAggregator::new(chain_id, api_key),
            solver_address,
        }
    }

    /// Check if filling an intent is profitable
    pub fn is_profitable(
        &self,
        input_amount: U256,
        min_output: U256,
        market_rate: U256,
    ) -> (bool, u64) {
        if market_rate <= min_output {
            return (false, 0);
        }

        let profit = market_rate - min_output;
        
        let profit_bps = if min_output > U256::ZERO {
            ((profit * U256::from(10000)) / min_output)
                .try_into()
                .unwrap_or(0u64)
        } else {
            0
        };

        let is_profitable = profit_bps >= self.min_profit_bps;

        info!(
            "📊 Profitability: input={}, min_output={}, market={}, profit_bps={}, ok={}",
            input_amount, min_output, market_rate, profit_bps, is_profitable
        );

        (is_profitable, profit_bps)
    }

    /// Get real market rate from 0x DEX aggregator
    pub async fn get_market_rate(
        &self,
        input_token: &str,
        output_token: &str,
        input_amount: U256,
    ) -> U256 {
        self.dex
            .get_quote_with_fallback(
                input_token,
                output_token,
                input_amount,
                &self.solver_address,
            )
            .await
    }
}
