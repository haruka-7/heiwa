use crate::cli::serve::AppState;
use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use tera::Context;
use pulldown_cmark::{Parser, Options, html};

pub async fn show(State(state): State<Arc<AppState>>) -> Html<String> {
    let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut context = Context::new();
    context.insert("title", "Title");
    context.insert("content", html_output.as_str());
    Html(state.tera.render("page.html", &context).unwrap())
}
