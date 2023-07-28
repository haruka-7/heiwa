use crate::services::jwt::auth;
use crate::templates::BackDashboardTemplate;
use axum::response::{IntoResponse, Redirect, Response};
use axum_auth::AuthBearer;

pub async fn show(AuthBearer(token): AuthBearer) -> Response {
    match auth(token) {
        Ok(_) => BackDashboardTemplate {
            name: "Dashboard".to_string(),
        }
        .into_response(),
        Err(_) => Redirect::to("/login").into_response(),
    }
}
