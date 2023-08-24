use crate::cli::serve::AppState;
use crate::entities::page::Page;
use axum::extract::{Host, State};
use axum::http::header::HeaderMap;
use axum::http::header::{self};
use axum::response::{IntoResponse, Response};
use chrono::prelude::*;
use glob::glob;
use sitewriter::{ChangeFreq, UrlEntry, UrlEntryBuilder};
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

    let mut urls = Vec::new();

    urls.push(
        UrlEntryBuilder::default()
            .loc(format!("https://{}", host).parse().unwrap())
            .build()
            .unwrap(),
    );

    for page in pages {
        println!("{}", page.date);
        urls.push(UrlEntry {
            loc: format!("https://{}/{}", host, page.url).parse().unwrap(),
            changefreq: Some(ChangeFreq::Weekly),
            priority: Some(1.0),
            lastmod: Some(DateTime::<Utc>::from_utc(
                NaiveDateTime::parse_from_str(page.date.as_str(), "%Y/%m/%d").unwrap(),
                Utc,
            )),
        });
    }

    let body: String = sitewriter::generate_str(&urls);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());
    (headers, body).into_response()
}
