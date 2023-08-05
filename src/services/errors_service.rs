use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use diesel::result::Error;
use serde_json::json;

/// TODO USE A GENERIC TYPE
pub fn handler_error(error: Error) -> Response {
    tracing::error!("{}", error);
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub fn handle_service_error(error: Option<String>) -> Response {
    match error {
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Some(code) => (StatusCode::BAD_REQUEST, Json(json!({"error": code}))).into_response(),
    }
}
