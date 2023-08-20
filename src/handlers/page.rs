use crate::cli::serve::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use tera::Context;

pub async fn show(Path(path): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("name", format!("Hidrile {}", path).as_str());
    Html(state.tera.render("home.html", &context).unwrap())
}
