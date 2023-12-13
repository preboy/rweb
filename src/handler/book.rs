use std::sync::Arc;

use axum::{extract::State, routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    // <Arc<AppState>>
    Router::new().route("/", get(root))
}

async fn root(State(app_state): State<Arc<AppState>>) -> &'static str {
    "Hello, World!"
}
