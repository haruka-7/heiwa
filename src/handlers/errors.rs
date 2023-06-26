use axum::http::StatusCode;
use axum::{BoxError, Json};
use serde_json::{Value, json};

pub async fn error(err: BoxError) -> (StatusCode, Json<Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({ "error" : "Request took too long".to_string()})),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error" : format!("Unhandled internal error: {}", err)})),
        )
    }
}
