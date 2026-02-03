use alloy::primitives::U256;
use tracing::info;

/// Profitability checker for deciding whether to fill an intent
pub struct ProfitabilityChecker {
    min_profit_bps: u64,
}

impl ProfitabilityChecker {
    pub fn new(min_profit_bps: u64) -> Self {
        Self { min_profit_bps }
    }

    /// Check if filling an intent is profitable
    /// Returns (is_profitable, expected_profit_bps)
    pub fn is_profitable(
        &self,
        input_amount: U256,
        min_output: U256,
        market_rate: U256, // How much we can actually get from DEX
    ) -> (bool, u64) {
        // Simple formula: 
        // profit = market_rate - min_output
        // profit_bps = (profit / min_output) * 10000
        
        if market_rate <= min_output {
            return (false, 0);
        }

        let profit = market_rate - min_output;
        
        // Calculate basis points (avoiding overflow)
        // profit_bps = (profit * 10000) / min_output
        let profit_bps = if min_output > U256::ZERO {
            ((profit * U256::from(10000)) / min_output)
                .try_into()
                .unwrap_or(0u64)
        } else {
            0
        };

        let is_profitable = profit_bps >= self.min_profit_bps;

        info!(
            "📊 Profitability check: input={}, min_output={}, market_rate={}, profit_bps={}, min_required={}, profitable={}",
            input_amount, min_output, market_rate, profit_bps, self.min_profit_bps, is_profitable
        );

        (is_profitable, profit_bps)
    }

    /// Mock market rate fetcher (in production, would call DEX aggregator)
    pub async fn get_market_rate(
        &self,
        _input_token: &str,
        _output_token: &str,
        input_amount: U256,
    ) -> U256 {
        // For demo: assume 1:1 rate with 2% slippage
        // In production: call 1inch, 0x, or Uniswap quoter
        input_amount * U256::from(98) / U256::from(100)
    }
}
