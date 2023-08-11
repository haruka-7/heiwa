use crate::services::articles_service::find_article_by_permalink;
use crate::templates::site_templates::{ArticleTemplate, ErrorPageTemplate};
use crate::AppState;
use axum::extract::{Path, State};
use std::sync::Arc;
use axum::response::{IntoResponse, Response};
use crate::services::authors_service::find_author_by_id;

pub async fn show(State(state): State<Arc<AppState>>, Path(permalink): Path<String>) -> Response {
    match find_article_by_permalink(&state, permalink) {
        Ok(article) => {
            let author_name: String = find_author_by_id(&state, article.author_id).unwrap().display_name;
            ArticleTemplate {
                title: article.title,
                content: article.content.unwrap_or("".to_string()),
                meta_title: article.meta_description.unwrap_or("".to_string()),
                author: author_name,
            }.into_response()
        }
        Err(_) => ErrorPageTemplate {}.into_response()
    }
}
