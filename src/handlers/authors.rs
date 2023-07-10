use crate::entities::authors::{
    verify_password, Author, LoginAuthor, LoginAuthorPassword, NewAuthor, UpdateAuthor,
};
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use heiwa_common::{handle_error, handler_validation_error};
use serde_json::json;
use validator::Validate;

pub async fn get(Path(name): Path<String>) -> Response {
    let author_result: QueryResult<Vec<Author>> = Author::find_by_name(name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                Json(json!((author.first().unwrap()))).into_response()
            }
        }
        Err(e) => handle_error(e),
    }
}

pub async fn create(Json(payload): Json<NewAuthor>) -> Response {
    match payload.validate() {
        Ok(_) => {
            let author_result: QueryResult<Author> = Author::create(payload);
            match author_result {
                Ok(author) => (StatusCode::CREATED, Json(json!(author))).into_response(),
                Err(e) => handle_error(e),
            }
        }
        Err(e) => handler_validation_error(e),
    }
}

pub async fn update(Json(payload): Json<UpdateAuthor>) -> Response {
    match payload.validate() {
        Ok(_) => {
            let update_result: QueryResult<usize> = Author::update(payload);
            match update_result {
                Ok(_) => StatusCode::OK.into_response(),
                Err(e) => handle_error(e),
            }
        }
        Err(e) => handler_validation_error(e),
    }
}

pub async fn login(Json(payload): Json<LoginAuthor>) -> Response {
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(payload.name);
    match author_result {
        Ok(author) => {
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
        Err(e) => handle_error(e),
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let delete_result: QueryResult<usize> = Author::delete(id);
    match delete_result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => handle_error(e),
    }
}
