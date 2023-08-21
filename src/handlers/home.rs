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
    context.insert("name", "Hidrile");

    let html = state.tera.render("home.html", &context).unwrap();
    Html(minify_html(html))
}
