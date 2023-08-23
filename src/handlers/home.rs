use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::template::minify_html;
use axum::extract::State;
use axum::response::Html;
use glob::glob;
use std::sync::Arc;
use tera::Context;

pub async fn show(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("meta_title", "Meta title");
    context.insert("meta_description", "Meta description");
    context.insert("site_title", state.config.title.as_str());
    context.insert("name", "Hidrile");

    let mut pages: Vec<Page> = Vec::new();
    for entry in glob("./pages/**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_path: String = path.into_os_string().into_string().unwrap();
                let url: String = file_path.replace("pages/", "").replace(".md", "");
                let page: Page = Page::new(url, file_path, state.mk_parser_options);
                if page.published {
                    pages.push(page);
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
