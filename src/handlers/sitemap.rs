use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::file::read_file;
use axum::extract::{Host, State};
use axum::http::header::HeaderMap;
use axum::http::header::{self};
use axum::response::{IntoResponse, Response};
use glob::glob;
use std::sync::Arc;

pub async fn show(Host(host): Host, State(state): State<Arc<AppState>>) -> Response {
    let mut pages: Vec<Page> = Vec::new();
    for entry in
        glob(&format!("{}/pages/**/*.md", state.path)).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                if path
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    != "home.md"
                {
                    let file_path: String = path.into_os_string().into_string().unwrap();
                    let file_content: String = read_file(&file_path);
                    let url: String = file_path
                        .rsplit("/pages/")
                        .next()
                        .unwrap()
                        .replace(".md", "");
                    let page: Page = Page::new(url, file_content, state.mk_parser_options);
                    if page.published {
                        pages.push(page);
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    let mut body = String::new();
    for page in pages {
        body += &format!("https://{}{}\n", host, page.url);
    }

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, body).into_response()
}
