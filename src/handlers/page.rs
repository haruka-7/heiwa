use crate::cli::serve::AppState;
use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use tera::Context;
use pulldown_cmark::{Parser, Options, html};
use minify_html::{Cfg, minify};

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

    let html = state.tera.render("page.html", &context).unwrap();
    let code: &[u8] = html.as_bytes();
    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.keep_spaces_between_attributes=true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    let minified_html = minify(&code, &cfg);

    Html(String::from_utf8(minified_html).unwrap())
}
