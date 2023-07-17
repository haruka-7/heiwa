use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use diesel::result::Error;
use serde_json::{json, Value};
use validator::ValidationErrors;

pub async fn error(err: BoxError) -> (StatusCode, Json<Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({ "error" : "Request took too long".to_string()})),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("Unhandled internal error: {}", err)
            })),
        )
    }
}

pub fn handle_error(error: Error) -> Response {
    tracing::error!("{}", error);
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub fn handler_validation_error(error: ValidationErrors) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(error))).into_response()
}