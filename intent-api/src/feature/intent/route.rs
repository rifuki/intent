use axum::routing::{Router, get, post};

use crate::{feature::intent::handler, state::AppState};

pub fn intent_route() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_intent))
        .route("/{id}", get(handler::get_intent))
        .route("/{id}/status", post(handler::cancel_intent))
}

pub fn chain_routes() -> Router<AppState> {
    Router::new().route("/", get(handler::list_chains))
}
