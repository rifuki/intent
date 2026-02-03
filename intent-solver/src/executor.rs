use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use tracing::{error, info};

use alloy::sol;

// Generate contract bindings
sol!(
    #[sol(rpc)]
    IntentGateway,
    "../intent-evm/contracts/out/IntentGateway.sol/IntentGateway.json"
);

pub struct IntentExecutor {
    gateway_address: Address,
    rpc_url: String,
    signer: PrivateKeySigner,
}

impl IntentExecutor {
    pub fn new(
        rpc_url: &str,
        gateway_address: &str,
        private_key: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let gateway_address: Address = gateway_address.parse()?;
        let signer: PrivateKeySigner = private_key.parse()?;

        Ok(Self {
            gateway_address,
            rpc_url: rpc_url.to_string(),
            signer,
        })
    }

    /// Fill an intent on-chain
    pub async fn fill_intent(
        &self,
        intent_id: U256,
        output_amount: U256,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!("💰 Executing fill for intent #{} with output {}", intent_id, output_amount);

        let url: Url = self.rpc_url.parse()?;
        let wallet = EthereumWallet::from(self.signer.clone());
        let provider = ProviderBuilder::new().wallet(wallet).connect_http(url);

        let contract = IntentGateway::new(self.gateway_address, provider);

        // Call fillIntent on contract
        let tx = contract
            .fillIntent(intent_id, output_amount)
            .send()
            .await
            .map_err(|e| {
                error!("❌ Failed to send fill tx: {}", e);
                e
            })?;

        let receipt = tx.get_receipt().await.map_err(|e| {
            error!("❌ Failed to get receipt: {}", e);
            e
        })?;

        let tx_hash = format!("{:?}", receipt.transaction_hash);
        info!("✅ Intent #{} filled! tx={}", intent_id, tx_hash);

        Ok(tx_hash)
    }

    pub fn solver_address(&self) -> Address {
        self.signer.address()
    }
}
