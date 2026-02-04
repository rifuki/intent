use axum::routing::{Router, get, post};

use crate::{feature::intent::handler, state::AppState};

pub fn intent_route() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_intent))
        .route("/{id}", get(handler::get_intent).delete(handler::cancel_intent))
        .route("/user/{address}", get(handler::get_user_intents))
}

pub fn chain_routes() -> Router<AppState> {
    Router::new().route("/", get(handler::list_chains))
}
