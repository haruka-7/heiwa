use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::entities::pagination::Pagination;
use crate::utils::file::read_file;
use axum::extract::{Query, State};
use axum::response::Html;
use glob::glob;
use minify::html::minify;
use std::sync::Arc;
use tera::Context;

pub async fn show(
    pagination: Option<Query<Pagination>>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let mut context = Context::new();
    context.insert("site_title", &state.config.title);
    context.insert(
        "mastodon_verification_link",
        &state.config.mastodon_verification_link,
    );
    context.insert("tags", &state.tags);

    let file_content: String = read_file(&format!("{}/pages/home.md", state.path));
    let home_page: Page = Page::new("/".to_string(), file_content, state.mk_parser_options);
    context.insert("meta_title", &home_page.title);
    context.insert("meta_description", &home_page.description);
    context.insert("home_content", &home_page.content);

    let mut pages: Vec<Page> = Vec::new();
    for entry in
        glob(&format!("{}/pages/**/*.md", state.path)).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let file_name: String = path
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                let file_path: String = path.into_os_string().into_string().unwrap();
                if file_name != "home.md" {
                    let file_content: String = read_file(&file_path);
                    let url: String = file_path
                        .rsplit("/pages/")
                        .next()
                        .unwrap()
                        .replace(".md", "");
                    let page: Page = Page::new(url, file_content, state.mk_parser_options);
                    if page.published && !page.title.is_empty() {
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
    match pagination {
        Some(params) => {
            let index: usize = params.page * state.config.articles_per_page;
            pages = pages.drain(index..).collect();
            if pages.len() > state.config.articles_per_page {
                context.insert("previous", &true);
            }
            pages.truncate(state.config.articles_per_page);
            let page_nb: usize = params.page + 1;
            context.insert("page_nb", &page_nb.to_string());
        }
        None => {
            if pages.len() > state.config.articles_per_page {
                pages.truncate(state.config.articles_per_page);
                context.insert("previous", &true);
                context.insert("page_nb", "1");
            }
        }
    }
    context.insert("pages", &pages);

    let html = state.tera.render("home.html", &context).unwrap();
    Html(minify(&html))
}
