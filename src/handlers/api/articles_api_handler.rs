use crate::entities::articles_entity::{Article, NewArticle};
use crate::entities::tags_entity::Tag;
use crate::services::articles_service::find_articles_by_author;
use crate::services::errors_service::{handler_error, handler_validation_errors};
use crate::services::jwt_service::verify;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum_auth::AuthBearer;
use diesel::QueryResult;
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

pub async fn get(State(state): State<Arc<AppState>>, Path(permalink): Path<String>) -> Response {
    let article_result: QueryResult<Vec<Article>> =
        Article::find_by_permalink(state.db_connection.get().unwrap(), permalink);
    match article_result {
        Ok(article) => {
            if article.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                Json(json!((article.first().unwrap()))).into_response()
            }
        }
        Err(e) => handler_error(e),
    }
}

pub async fn tag(
    State(state): State<Arc<AppState>>,
    Path(tag_permalink): Path<String>,
) -> Response {
    let tag_result: QueryResult<Vec<Tag>> = Tag::find_tag_by_permalink(tag_permalink);
    match tag_result {
        Ok(tag) => {
            let articles_result: QueryResult<Vec<Article>> =
                Article::find_by_tag(state.db_connection.get().unwrap(), tag.first().unwrap());
            match articles_result {
                Ok(articles) => {
                    if articles.is_empty() {
                        StatusCode::NOT_FOUND.into_response()
                    } else {
                        Json(json!((articles.first().unwrap()))).into_response()
                    }
                }
                Err(e) => handler_error(e),
            }
        }
        Err(e) => handler_error(e),
    }
}

pub async fn author(State(state): State<Arc<AppState>>, Path(author_id): Path<i32>) -> Response {
    match find_articles_by_author(&state, author_id) {
        Ok(articles) => Json(json!((articles))).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    token: AuthBearer,
    Json(payload): Json<NewArticle>,
) -> Response {
    match verify(token.0.as_str(), payload.author_id) {
        Ok(_) => match payload.validate() {
            Ok(_) => {
                let article: QueryResult<Article> =
                    Article::create(state.db_connection.get().unwrap(), payload);
                match article {
                    Ok(_) => StatusCode::CREATED.into_response(),
                    Err(e) => handler_error(e),
                }
            }
            Err(e) => handler_validation_errors(e),
        },
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

pub async fn delete(State(state): State<Arc<AppState>>, Path(id): Path<i32>) -> Response {
    let delete_result: QueryResult<usize> = Article::delete(state.db_connection.get().unwrap(), id);
    match delete_result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => handler_error(e),
    }
}
