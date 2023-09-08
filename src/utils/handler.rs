use axum::http::{header, StatusCode};
use axum::response::Response;

pub fn redirect_error_page() -> Response {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/error")
        .body(Default::default())
        .unwrap()
}

pub fn redirect_error_timeout() -> Response {
    Response::builder()
        .status(StatusCode::REQUEST_TIMEOUT)
        .header(header::LOCATION, "/error-page")
        .body(Default::default())
        .unwrap()
}
