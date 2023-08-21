use crate::cli::serve::AppState;
use crate::utils::markdown::markdown_to_html;
use crate::utils::template::minify_html;
use axum::extract::{Path, State};
use axum::response::Html;
use gray_matter::ParsedEntity;
use std::sync::Arc;
use tera::Context;

pub async fn show(Path(path): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let mut html_output: String = String::new();
    let result: ParsedEntity = markdown_to_html(&mut html_output, path, state.mk_parser_options);

    let mut context = Context::new();
    context.insert("meta_title", "Meta title");
    context.insert("meta_description", "Meta description");
    context.insert("site_title", state.config.title.as_str());
    context.insert(
        "title",
        result.data.unwrap()["title"].as_string().unwrap().as_str(),
    );
    context.insert("content", html_output.as_str());

    let html = state.tera.render("page.html", &context).unwrap();
    Html(minify_html(html))
}
