use axum::Router;

use crate::{
    feature::{
        health::route::health_routes,
        intent::route::{chain_routes, intent_route},
    },
    state::AppState,
};

pub fn app_routes(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/health", health_routes())
        .nest("/chains", chain_routes())
        .nest("/intents", intent_route());

    Router::new()
        .nest("/api/v1", api_routes)
        .fallback(common::handle_404)
        .with_state(state)
}

mod common {
    use axum::http::StatusCode;

    use crate::common::response::ApiErrorResponse;

    pub async fn handle_404() -> ApiErrorResponse {
        ApiErrorResponse::default()
            .with_code(StatusCode::NOT_FOUND)
            .with_message("The requested endpoint does not exist.")
    }
}
