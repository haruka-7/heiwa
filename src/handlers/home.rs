use crate::AppState;
use axum::extract::State;
use std::sync::Arc;
use axum::response::{Response, IntoResponse};

pub async fn show(State(state): State<Arc<AppState>>) -> Response {
    "Hello".to_string().into_response()
}
