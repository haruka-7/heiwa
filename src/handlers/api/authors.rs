use crate::entities::authors::{Author, NewAuthor, UpdateAuthor};
use crate::entities::roles::Roles;
use crate::handlers::api::errors::{handle_error, handler_validation_error};
use crate::services::authors::verify_password;
use crate::services::jwt;
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormLoginAuthor {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAuthor {
    pub token: String,
    pub role: String,
}

impl AuthAuthor {
    fn new(token: String, role: String) -> Self {
        Self { token, role }
    }
    pub(crate) fn default() -> Self {
        Self {
            token: "".to_string(),
            role: "".to_string(),
        }
    }
}

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
                Ok(author) => {
                    let jwt_token = jwt::sign(author.name).unwrap();
                    (
                        StatusCode::CREATED,
                        Json(json!({"access_token": jwt_token})),
                    )
                        .into_response()
                }
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

pub async fn login(Json(payload): Json<FormLoginAuthor>) -> Response {
    let author_result: QueryResult<Vec<Author>> = Author::find_by_name(payload.name);
    match author_result {
        Ok(authors) => {
            if authors.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                let author: &Author = authors.first().unwrap();
                match verify_password(&payload.password, &author.password) {
                    Ok(_) => {
                        let jwt_token = jwt::sign(author.name.clone()).unwrap();
                        Json(json!(AuthAuthor::new(
                            jwt_token,
                            author.role.clone().unwrap_or(Roles::Author.to_string())
                        )))
                        .into_response()
                    }
                    Err(_) => StatusCode::UNAUTHORIZED.into_response(),
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
