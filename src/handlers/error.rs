use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::BoxError;
use std::any::Any;
use crate::cli::serve::AppState;
use crate::utils::template::minify_html;
use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use tera::Context;

pub async fn show(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("meta_title", "Meta title");
    context.insert("meta_description", "Meta description");
    context.insert("site_title", state.config.title.as_str());
    let html = state.tera.render("error.html", &context).unwrap();
    Html(minify_html(html))
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
            .header(header::LOCATION, "/error")
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
        .header(header::LOCATION, "/error")
        .body(Default::default())
        .unwrap()
}
