use crate::cli::serve::AppState;
use crate::entities::page::Page;
use axum::extract::{Host, State};
use axum::http::header::HeaderMap;
use axum::http::header::{self};
use axum::response::{IntoResponse, Response};
use glob::glob;
use rss::{ChannelBuilder, Item, ItemBuilder};
use std::sync::Arc;

pub async fn show(Host(host): Host, State(state): State<Arc<AppState>>) -> Response {
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
                panic!("{}", e);
            }
        }
    }

    let mut items = Vec::new();
    for page in pages {
        let item: Item = ItemBuilder::default()
            .title(Some(page.title))
            .link(Some(format!("https://{}/{}", host, page.url)))
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
    headers.insert(header::CONTENT_TYPE, "application/rss+xml".parse().unwrap());
    (headers, channel.to_string()).into_response()
}
