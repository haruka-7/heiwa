use crate::entities::authors::{Author, LoginAuthor, LoginAuthorPassword, NewAuthor, verify_password};
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde_json::json;
use validator::Validate;

pub async fn get(Path(name): Path<String>) -> Response {
    let author: Vec<Author> = Author::find_by_name(name);
    if author.is_empty() {
        StatusCode::NOT_FOUND.into_response()
    } else {
        Json(json!((author.first().unwrap()))).into_response()
    }
}

pub async fn create(Json(payload): Json<NewAuthor>) -> Response {
    match payload.validate() {
        Ok(_) => {
            let author: QueryResult<Author> = Author::create(payload);
            match author {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => {
                    tracing::error!("{}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => Json(json!(e)).into_response(),
    }
}

pub async fn login(Json(payload): Json<LoginAuthor>) -> Response {
    let author: Vec<LoginAuthorPassword> =
        LoginAuthorPassword::find_by_name_for_login(payload.name);
    if author.is_empty() {
        StatusCode::NOT_FOUND.into_response()
    } else {
        let author: &LoginAuthorPassword = author.first().unwrap();
        if verify_password(payload.password, &author.password) {
            StatusCode::OK.into_response()
        } else {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let nb_deleted: usize = Author::delete(id);
    if nb_deleted == 0 {
        tracing::error!("Can not delete author with id {}", id);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
