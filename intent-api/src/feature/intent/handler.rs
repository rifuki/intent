use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use intent_core::{ChainAdapter, ChainId, ChainType, Intent, IntentStatus};

use crate::{
    common::response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse},
    feature::intent::dto::{ChainInfo, ChainsResponse, CreateIntentRequest, TxHashResponse},
    state::AppState,
};

pub async fn list_chains() -> ApiResponse<ChainsResponse> {
    let chains = vec![
        ChainInfo {
            id: ChainId::BaseSepolia,
            name: "Base Sepolia".to_string(),
            chain_type: "evm".to_string(),
        },
        ChainInfo {
            id: ChainId::ArbitrumSepolia,
            name: "Arbitrum Sepolia".to_string(),
            chain_type: "evm".to_string(),
        },
    ];

    Ok(ApiSuccessResponse::default().with_data(ChainsResponse { chains }))
}

pub async fn create_intent(
    State(state): State<AppState>,
    Json(payload): Json<CreateIntentRequest>,
) -> ApiResponse<TxHashResponse> {
    if payload.chain.chain_type() != ChainType::Evm {
        return Err(ApiErrorResponse::default()
            .with_code(StatusCode::BAD_REQUEST)
            .with_message("Only EVM chain are supported currently"));
    }

    let intent = Intent {
        id: None,
        chain: payload.chain,
        creator: payload.creator,
        input_token: payload.input_token,
        input_amount: payload.input_amount,
        output_token: payload.output_token,
        min_output_amount: payload.min_output_amount,
        deadline: payload.deadline,
        status: IntentStatus::Pending,
    };

    match state.evm_adapter.submit_intent(&intent).await {
        Ok(tx_hash) => Ok(ApiSuccessResponse::default()
            .with_code(StatusCode::CREATED)
            .with_message("Intent created successfully")
            .with_data(TxHashResponse { tx_hash })),
        Err(e) => Err(ApiErrorResponse::default()
            .with_code(StatusCode::INTERNAL_SERVER_ERROR)
            .with_message(&format!("Failed to create intent: {}", e))),
    }
}

pub async fn get_intent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiSuccessResponse<Intent>, ApiErrorResponse> {
    match state.evm_adapter.get_intent(&id).await {
        Ok(intent) => Ok(ApiSuccessResponse::default().with_data(intent)),
        Err(e) => Err(ApiErrorResponse::default()
            .with_code(StatusCode::NOT_FOUND)
            .with_message(&e.to_string())),
    }
}

pub async fn cancel_intent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiSuccessResponse<TxHashResponse>, ApiErrorResponse> {
    match state.evm_adapter.cancel_intent(&id).await {
        Ok(tx_hash) => Ok(ApiSuccessResponse::default()
            .with_message("Intent cancelled successfully")
            .with_data(TxHashResponse { tx_hash })),
        Err(e) => Err(ApiErrorResponse::default()
            .with_code(StatusCode::INTERNAL_SERVER_ERROR)
            .with_message(&e.to_string())),
    }
}
