use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diesel::result::Error;
use validator::ValidationErrors;

pub fn handle_error(error: Error) -> Response {
    tracing::error!("{}", error);
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub fn handler_validation_error(error: ValidationErrors) -> Response {
    tracing::warn!("{}", error);
    StatusCode::BAD_REQUEST.into_response()
}
