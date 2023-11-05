use axum::{extract::State, http::StatusCode, routing::get, Router};

use crate::app::AppState;

pub fn create_route<'a>() -> Router<AppState> {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check(State(_state): State<AppState>) -> StatusCode {
    StatusCode::OK
}
