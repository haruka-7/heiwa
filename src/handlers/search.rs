use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::file::read_file;
use crate::utils::template::minify_html;
use axum::extract::State;
use axum::response::Html;
use axum::Form;
use glob::glob;
use serde::Deserialize;
use std::sync::Arc;
use tera::Context;

#[derive(Deserialize)]
pub struct Search {
    keywords: String,
}

pub async fn show(State(state): State<Arc<AppState>>, Form(search): Form<Search>) -> Html<String> {
    let mut context = Context::new();
    context.insert("meta_title", "Search");
    context.insert("meta_description", "Search page");
    context.insert("site_title", &state.config.title);
    context.insert("search", &search.keywords);

    let mut pages: Vec<Page> = Vec::new();
    for entry in glob("./pages/**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_path: String = path.into_os_string().into_string().unwrap();
                let file_content: String = read_file(&file_path);
                if file_content
                    .to_lowercase()
                    .contains(&search.keywords.to_lowercase())
                {
                    let url: String = file_path.replace("pages/", "").replace(".md", "");
                    let page: Page = Page::new(url, file_content, state.mk_parser_options);
                    if page.published {
                        pages.push(page);
                    }
                }
            }
            Err(e) => {
                tracing::error!("{}", e);
                context.insert("alert", "No articles");
            }
        }
    }
    pages.sort_by(|a, b| b.date.cmp(&a.date));
    context.insert("pages", &pages);

    let html = state.tera.render("search.html", &context).unwrap();

    Html(minify_html(html))
}
