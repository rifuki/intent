use crate::common::response::{ApiResponse, ApiSuccessResponse};

pub async fn public_health_check() -> ApiResponse<()> {
    Ok(ApiSuccessResponse::default().with_message("Service is healthy"))
}
