use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::file::read_file;
use crate::utils::template::minify_html;
use axum::extract::State;
use axum::response::Html;
use glob::glob;
use std::sync::Arc;
use tera::Context;

pub async fn show(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("site_title", &state.config.title);
    context.insert("tags", &state.tags);

    let file_content: String = read_file(&"pages/home.md".to_string());
    let home_page: Page = Page::new("/".to_string(), file_content, state.mk_parser_options);
    context.insert("meta_title", &home_page.title);
    context.insert("meta_description", &home_page.description);
    context.insert("home_content", &home_page.content);

    let mut pages: Vec<Page> = Vec::new();
    for entry in glob("./pages/**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_path: String = path.into_os_string().into_string().unwrap();
                if file_path != "pages/home.md" {
                    let file_content: String = read_file(&file_path);
                    let url: String = file_path.replace("pages/", "").replace(".md", "");
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
    context.insert("pages", &pages);

    let html = state.tera.render("home.html", &context).unwrap();
    Html(minify_html(html))
}
