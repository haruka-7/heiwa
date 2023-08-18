use crate::cli::serve::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use tera::Context;

pub async fn show(State(state): State<Arc<AppState>>) -> Response {
    let mut context = Context::new();
    context.insert("name", "Hidrile");
    state
        .tera
        .render("home.html", &context)
        //.render(format!("{}/home.html", state.config.theme).as_str(), &context)
        .unwrap()
        .into_response()
}
