use crate::cli::serve::AppState;
use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use tera::Context;

pub async fn show(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("name", "Hidrile");
    Html(state.tera.render("home.html", &context).unwrap())
}
