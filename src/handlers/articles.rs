use crate::entities::articles::{Article, NewArticle};
use crate::entities::tags::Tag;
use crate::handlers::errors::handle_error;
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde_json::json;
use validator::Validate;

pub async fn get(Path(permalink): Path<String>) -> Response {
    let article_result: QueryResult<Vec<Article>> = Article::find_by_permalink(permalink);
    match article_result {
        Ok(article) => {
            if article.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                Json(json!((article.first().unwrap()))).into_response()
            }
        }
        Err(e) => handle_error(e),
    }
}

pub async fn tag(Path(tag_permalink): Path<String>) -> Response {
    let tag_result: QueryResult<Vec<Tag>> = Tag::find_tag_by_permalink(tag_permalink);
    match tag_result {
        Ok(tag) => {
            let articles_result: QueryResult<Vec<Article>> =
                Article::find_by_tag(tag.first().unwrap());
            match articles_result {
                Ok(articles) => {
                    if articles.is_empty() {
                        StatusCode::NOT_FOUND.into_response()
                    } else {
                        Json(json!((articles.first().unwrap()))).into_response()
                    }
                }
                Err(e) => handle_error(e),
            }
        }
        Err(e) => handle_error(e),
    }
}

pub async fn author(Path(author_id): Path<i32>) -> Response {
    let articles_result: QueryResult<Vec<Article>> = Article::find_by_author(author_id);
    match articles_result {
        Ok(articles) => {
            if articles.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                Json(json!((articles.first().unwrap()))).into_response()
            }
        }
        Err(e) => handle_error(e),
    }
}

pub async fn create(Json(payload): Json<NewArticle>) -> Response {
    match payload.validate() {
        Ok(_) => {
            let article: QueryResult<Article> = Article::create(payload);
            match article {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => handle_error(e),
            }
        }
        Err(e) => Json(json!(e)).into_response(),
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let delete_result: QueryResult<usize> = Article::delete(id);
    match delete_result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => handle_error(e),
    }
}
