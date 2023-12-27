use crate::cli::serve::AppState;
use crate::entities::page::Page;
use crate::utils::file::read_file;
use axum::extract::{Host, Query, State};
use axum::http::header::HeaderMap;
use axum::http::header::{self};
use axum::response::{IntoResponse, Response};
use glob::glob;
use rss::{ChannelBuilder, Item, ItemBuilder};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn show(
    Host(host): Host,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Response {
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
                if file_name != "home.md" {
                    let file_path: String = path.into_os_string().into_string().unwrap();
                    let file_content: String = read_file(&file_path);
                    let url: String = file_path
                        .rsplit("/pages/")
                        .next()
                        .unwrap()
                        .replace(".md", "");
                    let page: Page = Page::new(url, file_content, state.mk_parser_options);
                    if page.published {
                        match params.get(&"tag".to_string()) {
                            Some(tag) => {
                                if page.tags.contains(tag) {
                                    pages.push(page);
                                }
                            }
                            None => {
                                pages.push(page);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    pages.sort_by(|a, b| b.date.cmp(&a.date));

    let mut items = Vec::new();
    for page in pages {
        let item: Item = ItemBuilder::default()
            .title(Some(page.title))
            .link(Some(format!("https://{}{}", host, page.url)))
            .author(Some(page.author))
            .description(Some(page.description))
            .pub_date(Some(page.date))
            .content(Some(page.content))
            .build();
        items.push(item);
    }
    let channel = ChannelBuilder::default()
        .title(&state.config.title)
        .link(format!("https://{}", host))
        .items(items)
        .build();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/xml".parse().unwrap());
    (headers, channel.to_string()).into_response()
}
