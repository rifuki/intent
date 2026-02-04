use std::sync::Arc;
use std::time::Duration;

use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use tokio::time::sleep;
use tracing::{error, info, warn};

use alloy::sol;

use crate::executor::IntentExecutor;
use crate::strategy::ProfitabilityChecker;
use intent_db::sea_orm::DatabaseConnection;
use intent_db::repo::{IntentRepoImpl, IntentRepository};
use intent_db::entity::intent::{self, Entity as IntentEntity};
use intent_db::sea_orm::*;

// Generate contract bindings
sol!(
    #[sol(rpc)]
    IntentGateway,
    "../intent-evm/contracts/out/IntentGateway.sol/IntentGateway.json"
);

pub struct IntentMonitor {
    rpc_url: String,
    gateway_address: Address,
    strategy: Arc<ProfitabilityChecker>,
    executor: Arc<IntentExecutor>,
    db: DatabaseConnection,
    last_intent_id: U256,
}

impl IntentMonitor {
    pub fn new(
        rpc_url: &str,
        gateway_address: &str,
        strategy: Arc<ProfitabilityChecker>,
        executor: Arc<IntentExecutor>,
        db: DatabaseConnection,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let gateway_address: Address = gateway_address.parse()?;

        Ok(Self {
            rpc_url: rpc_url.to_string(),
            gateway_address,
            strategy,
            executor,
            db,
            last_intent_id: U256::ZERO,
        })
    }

    /// Main monitoring loop
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url: Url = self.rpc_url.parse()?;
        let provider = ProviderBuilder::new().connect_http(url);
        let contract = IntentGateway::new(self.gateway_address, provider);

        info!("🔍 Monitoring for new intents...");

        let mut current_id = self.last_intent_id;

        loop {
            // Get next intent count
            match contract.nextIntentId().call().await {
                Ok(next_id) => {
                    // next_id is U256 directly

                    // Check for new intents
                    while current_id < next_id {
                        info!("📬 Found new intent #{}", current_id);

                        // Get intent details
                        match contract.getIntent(current_id).call().await {
                            Ok(intent) => {
                                // intent is Intent struct directly
                                
                                // Indexing: Save to DB
                                let repo = IntentRepoImpl::new(self.db.clone());
                                let id_i64 = current_id.to_string().parse::<i64>().unwrap_or_default();
                                
                                if let Ok(None) = repo.find_by_id(id_i64).await {
                                    let new_intent = intent_db::entity::intent::Model {
                                        id: id_i64,
                                        creator: format!("{:?}", intent.creator),
                                        input_token: format!("{:?}", intent.inputToken),
                                        input_amount: intent.inputAmount.to_string(),
                                        output_token: format!("{:?}", intent.outputToken),
                                        min_output_amount: intent.minOutputAmount.to_string(),
                                        deadline: intent.deadline.to_string().parse::<i64>().unwrap_or(0),
                                        status: intent.status as i16,
                                        created_at: chrono::Utc::now().naive_utc(),
                                        updated_at: chrono::Utc::now().naive_utc(),
                                    };
                                    
                                    if let Err(e) = repo.create(new_intent).await {
                                        error!("❌ Failed to save intent #{} to DB: {}", current_id, e);
                                    } else {
                                        info!("💾 Saved intent #{} to DB", current_id);
                                    }
                                }

                                // Only process pending intents (status == 0)
                                if intent.status == 0 {
                                    self.process_intent(current_id, &intent).await;
                                } else {
                                    info!("⏭️ Intent #{} already processed (status={:?})", current_id, intent.status);
                                }
                            }
                            Err(e) => {
                                error!("❌ Failed to get intent #{}: {}", current_id, e);
                            }
                        }

                        current_id += U256::from(1);
                    }
                }
                Err(e) => {
                    warn!("⚠️ Failed to get next intent ID: {}", e);
                }
            }

            // Sleep before next poll
            sleep(Duration::from_secs(5)).await;
        }
    }

    async fn process_intent(&self, intent_id: U256, intent: &IntentGateway::Intent) {
        info!(
            "🔎 Processing intent #{}: {} -> {}, min_output={}",
            intent_id,
            intent.inputToken,
            intent.outputToken,
            intent.minOutputAmount
        );

        // Get market rate
        let market_rate = self
            .strategy
            .get_market_rate(
                &format!("{:?}", intent.inputToken),
                &format!("{:?}", intent.outputToken),
                intent.inputAmount,
            )
            .await;

        // Check profitability
        let (is_profitable, profit_bps) = self.strategy.is_profitable(
            intent.inputAmount,
            intent.minOutputAmount,
            market_rate,
        );

        if is_profitable {
            info!(
                "✨ Intent #{} is profitable ({} bps)! Attempting fill...",
                intent_id, profit_bps
            );

            match self.executor.fill_intent(intent_id, intent.outputToken, market_rate).await {
                Ok(tx_hash) => {
                    info!("🎉 Successfully filled intent #{}: {}", intent_id, tx_hash);
                    
                    // Indexing: Update status to Filled (1)
                    let repo = IntentRepoImpl::new(self.db.clone());
                    let id_i64 = intent_id.to_string().parse::<i64>().unwrap_or_default();
                    if let Err(e) = repo.update_status(id_i64, 1).await {
                         error!("❌ Failed to update DB status for intent #{}: {}", intent_id, e);
                    } else {
                         info!("💾 Updated intent #{} status to FILLED", intent_id);
                    }
                }
                Err(e) => {
                    error!("❌ Failed to fill intent #{}: {}", intent_id, e);
                }
            }
        } else {
            info!(
                "⏭️ Intent #{} not profitable enough ({} bps < {} bps required)",
                intent_id, profit_bps, 50
            );
        }
    }
}
