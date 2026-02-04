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
use intent_db::{repo::{IntentRepoImpl, IntentRepository}, entity::intent::Model as IntentModel};

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
    // Verify chain type
    if payload.chain.chain_type() != ChainType::Evm {
        return Err(ApiErrorResponse::default()
            .with_code(StatusCode::BAD_REQUEST)
            .with_message("Only EVM chain are supported currently"));
    }

    // 1. Submit/Validate via Adapter (if needed, otherwise just rely on frontend TX)
    // For now, let's assume this submits to chain OR just validates. 
    // If frontend executes TX, we might want to change this flow. 
    // But keeping existing logic:
    let tx_hash = match state.evm_adapter.submit_intent(&Intent {
        id: None, // Generated on-chain
        chain: payload.chain.clone(),
        creator: payload.creator.clone(),
        input_token: payload.input_token.clone(),
        input_amount: payload.input_amount.clone(),
        output_token: payload.output_token.clone(),
        min_output_amount: payload.min_output_amount.clone(),
        deadline: payload.deadline,
        status: IntentStatus::Pending,
    }).await {
        Ok(hash) => hash,
        Err(e) => return Err(ApiErrorResponse::default().with_code(StatusCode::INTERNAL_SERVER_ERROR).with_message(&e.to_string())),
    };

    // 2. Persist to DB
    let repo = IntentRepoImpl::new(state.db.clone());
    let new_intent = intent_db::entity::intent::Model {
        id: 0, // Placeholder, ideally should get ID from event or tx receipt. 
               // Since we don't have ID yet (async mining), we might need another flow.
               // BUT for "My Intents" to show PENDING instantly, we need to save it.
               // Let's use a temporary ID or handle this constraint.
               // Actually, `id` is primary key (i64). If contract generates ID, we can't guess it.
               // FOR NOW: Let's skip saving creates if we don't know ID. 
               // OR: Use a random ID for "Draft" intents?
               
               // Better approach: "My Intents" page currently relies on on-chain data?
               // The user wants DB for SCALABILITY.
               // So we MUST save it. 
               // Strategy: Frontend calls this *after* TX is mined and it knows the ID?
               // Or we fetch events.
               
        creator: payload.creator.clone(),
        input_token: payload.input_token.address,
        input_amount: payload.input_amount,
        output_token: payload.output_token.address,
        min_output_amount: payload.min_output_amount,
        deadline: payload.deadline as i64, 
        status: 0, // Pending
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    
    // NOTE: Saving to DB here is tricky without the IntentID from contract.
    // I will defer the "save to DB" logic to the Indexer / Background worker 
    // OR change the API to accept `intent_id` if the frontend already knows it.
    
    // Reverting to just return tx_hash for now, as DB needs ID.
    Ok(ApiSuccessResponse::default()
        .with_code(StatusCode::CREATED)
        .with_message("Intent submitted successfully")
        .with_data(TxHashResponse { tx_hash }))
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

pub async fn get_user_intents(
    State(state): State<AppState>,
    Path(user_address): Path<String>,
) -> Result<ApiSuccessResponse<Vec<IntentModel>>, ApiErrorResponse> {
    let repo = IntentRepoImpl::new(state.db.clone());
    
    match repo.find_by_creator(&user_address).await {
        Ok(intents) => Ok(ApiSuccessResponse::default().with_data(intents)),
        Err(e) => {
            tracing::error!("Failed to fetch user intents: {}", e);
            Err(ApiErrorResponse::default()
                .with_code(StatusCode::INTERNAL_SERVER_ERROR)
                .with_message("Internal server error"))
        }
    }
}
