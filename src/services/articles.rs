use crate::entities::articles::{Article, NewArticle};
use axum::http::StatusCode;
use axum_sessions::extractors::WritableSession;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use crate::services::http_client::build_http_client;

#[derive(Debug, Deserialize)]
pub struct Articles {
    pub articles: Vec<Article>
}

impl Articles {
    pub fn default() -> Self {
        Self {
            articles: Vec::new()
        }
    }
}

pub async fn list_articles_api_call(author_id: i32) -> Result<Vec<Article>, ()> {
    let client = build_http_client();
    let response = client
        .get(format!("http://localhost:3000/api/articles/author/{}", author_id.to_string()))
        .send().await.unwrap();
    if response.status() == StatusCode::OK{
        Ok(response.json::<Vec<Article>>().await.unwrap_or(vec![]))
    } else {
        Err(())
    }
}

pub async fn create_article_api_call(
    session: &mut WritableSession,
    new_article: NewArticle,
) -> Result<(), ()> {
    let client = build_http_client();
    let request = client
        .post("http://localhost:3000/api/articles/create")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", session.get::<String>("token").unwrap()),
        )
        .json(&new_article);
    let response = request.send().await.unwrap();
    tracing::debug!(
        "\nREQUEST POST http://localhost:3000/api/articles/create \n{:?}\nRESPONSE\n{:?}",
        &new_article,
        &response
    );
    if response.status() == StatusCode::CREATED {
        Ok(())
    } else {
        Err(())
    }
}
