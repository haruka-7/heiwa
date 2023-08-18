use crate::cli::serve::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

pub async fn show(State(state): State<Arc<AppState>>) -> Response {
    "Hello".to_string().into_response()
}
