use crate::entities::authors::{Author, NewAuthor};
use crate::services::authors::{create_author, delete_author, get_authors, get_authors_by_name};
use crate::utils::establish_connection;
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;
use validator::Validate;

pub async fn authors() -> Response {
    let authors: Vec<Author> = get_authors(&mut establish_connection());
    if authors.is_empty() {
        StatusCode::NOT_FOUND.into_response()
    } else {
        Json(json!(authors)).into_response()
    }
}

pub async fn author(Path(name): Path<String>) -> Response {
    let author: Vec<Author> = get_authors_by_name(&mut establish_connection(), name);
    if author.is_empty() {
        StatusCode::NOT_FOUND.into_response()
    } else {
        Json(json!((author.first().unwrap()))).into_response()
    }
}

pub async fn create(Json(payload): Json<NewAuthor>) -> Response {
    match payload.validate() {
        Ok(_) => {
            let author: Author = create_author(&mut establish_connection(), payload);
            if author.name.is_empty() {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                StatusCode::OK.into_response()
            }
        }
        Err(_e) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let nb_deleted: usize = delete_author(&mut establish_connection(), id);
    if nb_deleted == 0 {
        tracing::error!("Can not delete author with id {}", id);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
