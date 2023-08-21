use crate::cli::serve::AppState;
use crate::utils::markdown::markdown_parser;
use axum::extract::{Path, State};
use axum::response::Html;
use minify_html::{minify, Cfg};
use pulldown_cmark::html;
use std::sync::Arc;
use tera::Context;

pub async fn show(Path(path): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let mut html_output = String::new();
    html::push_html(
        &mut html_output,
        markdown_parser(path, state.mk_parser_options),
    );

    let mut context = Context::new();
    context.insert("meta_title", "Title");
    context.insert("meta_description", "description");
    context.insert("site_title", "Site title");
    context.insert("title", "Article title");
    context.insert("content", html_output.as_str());

    let html = state.tera.render("page.html", &context).unwrap();
    let code: &[u8] = html.as_bytes();
    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.keep_closing_tags = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    let minified_html = minify(code, &cfg);

    Html(String::from_utf8(minified_html).unwrap())
}
