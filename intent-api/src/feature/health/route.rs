use axum::routing::{Router, get};

use crate::{feature::health::handler, state::AppState};

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/", get(handler::public_health_check))
}
