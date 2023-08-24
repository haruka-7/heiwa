use crate::cli::serve::AppState;
use crate::entities::page::Page;
use axum::extract::State;
use axum::http::header::HeaderMap;
use axum::http::header::{self};
use axum::response::{IntoResponse, Response};
use chrono::prelude::*;
use glob::glob;
use sitewriter::{ChangeFreq, UrlEntry, UrlEntryBuilder};
use std::sync::Arc;

pub async fn show(State(state): State<Arc<AppState>>) -> Response {
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

    let mut urls = Vec::new();

    urls.push(
        UrlEntryBuilder::default()
            .loc("https://localhost".parse().unwrap())
            .build()
            .unwrap(),
    );

    for page in pages {
        urls.push(UrlEntry {
            loc: format!("https://localhost/{}", page.url).parse().unwrap(),
            changefreq: Some(ChangeFreq::Weekly),
            priority: Some(1.0),
            lastmod: Some(Utc::now()),
        });
    }

    let body: String = sitewriter::generate_str(&urls);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());
    (headers, body).into_response()
}