use crate::entities::authors::{validate_unique_name, Author, NewAuthor, UpdateAuthor};
use crate::entities::roles::Roles;
use crate::handlers::api::errors::{
    handle_error, handler_validation_error, handler_validation_errors,
};
use crate::services::authors::verify_password;
use crate::services::jwt;
use crate::services::jwt::verify;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum_auth::AuthBearer;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormLoginAuthor {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAuthor {
    pub id: i32,
    pub token: String,
    pub role: String,
}

impl AuthAuthor {
    fn new(id: i32, token: String, role: String) -> Self {
        Self { id, token, role }
    }
    pub(crate) fn default() -> Self {
        Self {
            id: 0,
            token: "".to_string(),
            role: "".to_string(),
        }
    }
}

pub async fn get(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Response {
    let author_result: QueryResult<Vec<Author>> =
        Author::find_by_name(state.db_connection.get().unwrap(), name);
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

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FormLoginAuthor>,
) -> Response {
    let author_result: QueryResult<Vec<Author>> =
        Author::find_by_name(state.db_connection.get().unwrap(), payload.name);
    match author_result {
        Ok(authors) => {
            if authors.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                let author: &Author = authors.first().unwrap();
                match verify_password(&payload.password, &author.password) {
                    Ok(_) => {
                        let jwt_token = jwt::sign(author.id).unwrap();
                        Json(json!(AuthAuthor::new(
                            author.id,
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

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewAuthor>,
) -> Response {
    match payload.validate() {
        Ok(_) => match validate_unique_name(state, &payload.name) {
            Ok(_) => {
                let author_result: QueryResult<Author> = Author::create(payload);
                match author_result {
                    Ok(author) => {
                        let jwt_token = jwt::sign(author.id).unwrap();
                        (
                            StatusCode::CREATED,
                            Json(json!({ "access_token": jwt_token })),
                        )
                            .into_response()
                    }
                    Err(e) => handle_error(e),
                }
            }
            Err(e) => handler_validation_error(e),
        },
        Err(e) => handler_validation_errors(e),
    }
}

/// TODO add the author id as param to the verify function
pub async fn update(token: AuthBearer, Json(payload): Json<UpdateAuthor>) -> Response {
    match verify(token.0.as_str(), payload.id) {
        Ok(_) => match payload.validate() {
            Ok(_) => {
                let update_result: QueryResult<usize> = Author::update(payload);
                match update_result {
                    Ok(_) => StatusCode::OK.into_response(),
                    Err(e) => handle_error(e),
                }
            }
            Err(e) => handler_validation_errors(e),
        },
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

pub async fn delete(token: AuthBearer, Path(id): Path<i32>) -> Response {
    match verify(token.0.as_str(), id) {
        Ok(_) => {
            let delete_result: QueryResult<usize> = Author::delete(id);
            match delete_result {
                Ok(_) => StatusCode::OK.into_response(),
                Err(e) => handle_error(e),
            }
        }
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}
