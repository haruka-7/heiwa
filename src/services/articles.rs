use crate::entities::articles::NewArticle;
use axum::http::StatusCode;
use axum_sessions::extractors::WritableSession;
use reqwest::header::AUTHORIZATION;
use crate::services::http_client::build_http_client;

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
