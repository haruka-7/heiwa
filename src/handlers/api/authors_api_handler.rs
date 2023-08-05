use crate::entities::authors_entity::{Author, NewAuthor, UpdateAuthor};
use crate::services::authors_service::{
    auth_author, create_author, find_author_by_name, update_author,
};
use crate::services::errors_service::{handle_service_error, handler_error};
use crate::services::jwt_service::verify;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum_auth::AuthBearer;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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
    pub fn new(id: i32, token: String, role: String) -> Self {
        Self { id, token, role }
    }
}

// TODO SECURITY this endpoint return the author hashed password
pub async fn get(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Response {
    match find_author_by_name(&state, name) {
        Ok(author) => Json(json!(author)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FormLoginAuthor>,
) -> Response {
    match auth_author(&state, payload) {
        Ok(auth_author) => Json(json!(auth_author)).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewAuthor>,
) -> Response {
    match create_author(&state, payload) {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(error_code) => handle_service_error(error_code),
    }
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    token: AuthBearer,
    Json(payload): Json<UpdateAuthor>,
) -> Response {
    match verify(token.0.as_str(), payload.id) {
        Ok(_) => match update_author(&state, payload) {
            Ok(_) => StatusCode::OK.into_response(),
            Err(error_code) => handle_service_error(error_code),
        },
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

// TODO use a delete author service instead
pub async fn delete(
    State(state): State<Arc<AppState>>,
    token: AuthBearer,
    Path(id): Path<i32>,
) -> Response {
    match verify(token.0.as_str(), id) {
        Ok(_) => {
            let delete_result: QueryResult<usize> =
                Author::delete(state.db_connection.get().unwrap(), id);
            match delete_result {
                Ok(_) => StatusCode::OK.into_response(),
                Err(e) => handler_error(e),
            }
        }
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}
