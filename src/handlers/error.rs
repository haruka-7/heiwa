use crate::cli::serve::AppState;
use crate::utils::handler::{redirect_error_page, redirect_error_timeout};
use crate::utils::template::minify_html;
use axum::extract::State;
use axum::response::Html;
use axum::response::Response;
use axum::BoxError;
use std::any::Any;
use std::sync::Arc;
use tera::Context;

pub async fn show(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("meta_title", "Error");
    context.insert("meta_description", "Error page");
    context.insert("site_title", state.config.title.as_str());
    let html = state.tera.render("error.html", &context).unwrap();
    Html(minify_html(html))
}

pub async fn error(err: BoxError) -> Response {
    if err.is::<tower::timeout::error::Elapsed>() {
        tracing::error!(err);
        redirect_error_timeout()
    } else {
        tracing::error!(err);
        redirect_error_page()
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
    redirect_error_page()
}
