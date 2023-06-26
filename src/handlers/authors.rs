use crate::entities::authors::{Author, LoginAuthor, LoginAuthorPassword, NewAuthor};
use crate::services::authors::{create_author, delete_author, get_authors_by_name, get_authors_by_name_for_login, verify_password};
use crate::utils::establish_connection;
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;
use validator::Validate;

pub async fn get(Path(name): Path<String>) -> Response {
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

pub async fn login(Json(payload): Json<LoginAuthor>) -> Response {
    let author: Vec<LoginAuthorPassword> = get_authors_by_name_for_login(&mut establish_connection(), payload.name);
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
    let nb_deleted: usize = delete_author(&mut establish_connection(), id);
    if nb_deleted == 0 {
        tracing::error!("Can not delete author with id {}", id);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
