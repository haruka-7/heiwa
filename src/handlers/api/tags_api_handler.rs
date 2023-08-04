use crate::entities::articles_entity::Article;
use crate::entities::articles_tags_entity::ArticleTag;
use crate::entities::tags_entity::{NewTag, Tag};
use crate::services::errors_service::handler_error;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde_json::json;
use std::sync::Arc;

pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(article_permalink): Path<String>,
) -> Response {
    let article_result: QueryResult<Vec<Article>> =
        Article::find_by_permalink(state.db_connection.get().unwrap(), article_permalink);
    match article_result {
        Ok(article) => {
            if article.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                let tags_result: QueryResult<Vec<Tag>> =
                    Tag::find_tags_by_article(article.first().unwrap());
                match tags_result {
                    Ok(tags) => Json(json!(tags)).into_response(),
                    Err(e) => handler_error(e),
                }
            }
        }
        Err(e) => handler_error(e),
    }
}

pub async fn create(Path(article_id): Path<i32>, Json(payload): Json<NewTag>) -> Response {
    let tag_result: QueryResult<Tag> = Tag::create(payload);
    match tag_result {
        Ok(tag) => {
            let article_tag: ArticleTag = ArticleTag {
                article_id,
                tag_id: tag.id,
            };
            let article_tag_result: QueryResult<ArticleTag> = ArticleTag::create(article_tag);
            match article_tag_result {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => {
                    tracing::error!("Can not insert article-tag relation with error {}", e);
                    // Rollback Tag insertion if ArticleTag can not be inserted
                    let delete_result: QueryResult<usize> = Tag::delete(tag.id);
                    match delete_result {
                        Ok(_) => {}
                        Err(e) => {
                            return handler_error(e);
                        }
                    }
                    handler_error(e)
                }
            }
        }
        Err(e) => handler_error(e),
    }
}

pub async fn delete(Path(article_id): Path<i32>, Path(tag_id): Path<i32>) -> Response {
    let delete_result: QueryResult<usize> = ArticleTag::delete(article_id, tag_id);
    match delete_result {
        Ok(nb_relations_deleted) => {
            if nb_relations_deleted == 0 {
                tracing::error!(
                    "Can not delete article-tag relation with article_id {} and tag_id {}",
                    article_id,
                    tag_id
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                StatusCode::OK.into_response()
            }
        }
        Err(e) => handler_error(e),
    }
}
