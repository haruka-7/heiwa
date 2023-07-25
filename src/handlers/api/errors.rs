use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use diesel::result::Error;
use serde_json::json;
use validator::ValidationErrors;

pub fn handle_error(error: Error) -> Response {
    tracing::error!("{}", error);
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub fn handler_validation_error(error: ValidationErrors) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(error))).into_response()
}
