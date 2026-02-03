mod bindings;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use async_trait::async_trait;
use intent_core::{
    AdapterError, ChainAdapter, ChainId, ChainType, Intent, IntentStatus, TokenInfo,
};

use bindings::IntentGateway;

pub struct EvmAdapter {
    chain: ChainId,
    rpc_url: String,
    contract_address: Address,
    signer: Option<PrivateKeySigner>,
}

impl EvmAdapter {
    pub fn new(
        chain: ChainId,
        rpc_url: &str,
        contract_address: &str,
        private_key: Option<&str>,
    ) -> Result<Self, AdapterError> {
        if chain.chain_type() != ChainType::Evm {
            return Err(AdapterError::Rpc(format!("{chain:?} is not an EVM chain")));
        }

        let contract_address = contract_address
            .parse::<Address>()
            .map_err(|e| AdapterError::Rpc(format!("Invalid contract address: {e}")))?;

        let signer = private_key
            .map(|pk| {
                pk.parse::<PrivateKeySigner>()
                    .map_err(|e| AdapterError::Rpc(format!("Invalid private key: {e}")))
            })
            .transpose()?;

        Ok(Self {
            chain,
            rpc_url: rpc_url.to_string(),
            contract_address,
            signer,
        })
    }

    /// Create a read-only provider
    fn provider(&self) -> Result<impl Provider + Clone, AdapterError> {
        let url: Url = self
            .rpc_url
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid RPC URL: {e}")))?;

        Ok(ProviderBuilder::new().connect_http(url))
    }

    /// Create a provider with wallet for signing transactions
    fn signer_provider(&self) -> Result<impl Provider + Clone, AdapterError> {
        let signer = self
            .signer
            .clone()
            .ok_or(AdapterError::Rpc("No signer configured".into()))?;

        let url: Url = self
            .rpc_url
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid RPC URL: {e}")))?;

        let wallet = EthereumWallet::from(signer);

        Ok(ProviderBuilder::new().wallet(wallet).connect_http(url))
    }
}

#[async_trait]
impl ChainAdapter for EvmAdapter {
    fn chain_id(&self) -> ChainId {
        self.chain
    }

    async fn submit_intent(&self, intent: &Intent) -> Result<String, AdapterError> {
        // Parse addresses and amounts
        let input_token: Address = intent
            .input_token
            .address
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid input token address: {e}")))?;

        let output_token: Address = intent
            .output_token
            .address
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid output token address: {e}")))?;

        let input_amount: U256 = intent
            .input_amount
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid input amount: {e}")))?;

        let min_output: U256 = intent
            .min_output_amount
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid min output amount: {e}")))?;

        let deadline = U256::from(intent.deadline);

        // Create provider with signer
        let provider = self.signer_provider()?;
        let contract = IntentGateway::new(self.contract_address, provider);

        // Call createIntent on contract
        let tx = contract
            .createIntent(input_token, input_amount, output_token, min_output, deadline)
            .send()
            .await
            .map_err(|e| AdapterError::TxFailed(format!("Failed to send tx: {e}")))?;

        // Wait for receipt
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| AdapterError::TxFailed(format!("Failed to get receipt: {e}")))?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }

    async fn get_intent(&self, intent_id: &str) -> Result<Intent, AdapterError> {
        // Parse intent ID to U256
        let id: U256 = intent_id
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid intent ID: {e}")))?;

        // Create provider and contract instance
        let provider = self.provider()?;
        let contract = IntentGateway::new(self.contract_address, provider);

        // Call getIntent on contract
        let on_chain = contract
            .getIntent(id)
            .call()
            .await
            .map_err(|e| AdapterError::Rpc(e.to_string()))?;

        // Map status (0=Pending, 1=Filled, 2=Cancelled)
        let status = match on_chain.status {
            0 => IntentStatus::Pending,
            1 => IntentStatus::Filled,
            2 => IntentStatus::Cancelled,
            _ => IntentStatus::Pending,
        };

        Ok(Intent {
            id: Some(intent_id.to_string()),
            chain: self.chain,
            creator: format!("{:?}", on_chain.creator),
            input_token: TokenInfo {
                address: format!("{:?}", on_chain.inputToken),
                symbol: "UNKNOWN".to_string(),
                decimals: 18,
            },
            input_amount: on_chain.inputAmount.to_string(),
            output_token: TokenInfo {
                address: format!("{:?}", on_chain.outputToken),
                symbol: "UNKNOWN".to_string(),
                decimals: 18,
            },
            min_output_amount: on_chain.minOutputAmount.to_string(),
            deadline: on_chain.deadline.try_into().unwrap_or(0),
            status,
        })
    }

    async fn cancel_intent(&self, intent_id: &str) -> Result<String, AdapterError> {
        // Parse intent ID
        let id: U256 = intent_id
            .parse()
            .map_err(|e| AdapterError::Rpc(format!("Invalid intent ID: {e}")))?;

        // Create provider with signer
        let provider = self.signer_provider()?;
        let contract = IntentGateway::new(self.contract_address, provider);

        // Call cancelIntent on contract
        let tx = contract
            .cancelIntent(id)
            .send()
            .await
            .map_err(|e| AdapterError::TxFailed(format!("Failed to send tx: {e}")))?;

        // Wait for receipt
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| AdapterError::TxFailed(format!("Failed to get receipt: {e}")))?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }
}
