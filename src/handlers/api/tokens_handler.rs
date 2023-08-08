use crate::services::jwt_service::verify;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_auth::AuthBearer;

pub async fn token_verify(token: AuthBearer, Path(author_id): Path<i32>) -> Response {
    match verify(token.0.as_str(), author_id) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}
