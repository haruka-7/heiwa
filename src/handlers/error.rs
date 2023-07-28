use crate::templates::ErrorPageTemplate;
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::BoxError;
use std::any::Any;

pub async fn show() -> ErrorPageTemplate {
    ErrorPageTemplate {}
}

pub async fn error(err: BoxError) -> Response {
    if err.is::<tower::timeout::error::Elapsed>() {
        tracing::error!(err);
        Response::builder()
            .status(StatusCode::REQUEST_TIMEOUT)
            .header(header::LOCATION, "/error-page")
            .body(Default::default())
            .unwrap()
    } else {
        tracing::error!(err);
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::LOCATION, "/error-page")
            .body(Default::default())
            .unwrap()
    }
}

pub fn panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };
    tracing::error!(details);
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/error-page")
        .body(Default::default())
        .unwrap()
}
