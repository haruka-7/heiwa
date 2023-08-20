use crate::cli::serve::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use std::sync::Arc;
use tera::Context;
use pulldown_cmark::{Parser, Options, html};
use minify_html::{Cfg, minify};
use std::fs::read_to_string;

pub async fn show(Path(path): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {

    let markdown_input = read_to_string(format!("./pages/{}.md", path.clone())).expect(format!("Should have been able to read the file : {}", path).as_str());

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(markdown_input.as_str(), options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut context = Context::new();
    context.insert("title", "Title");
    context.insert("content", html_output.as_str());

    let html = state.tera.render("page.html", &context).unwrap();
    let code: &[u8] = html.as_bytes();
    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.keep_closing_tags = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_spaces_between_attributes=true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    let minified_html = minify(&code, &cfg);

    Html(String::from_utf8(minified_html).unwrap())
}
