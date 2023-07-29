use crate::services::jwt::auth;
use crate::templates::BackDashboardTemplate;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::ReadableSession;

pub async fn show(session: ReadableSession) -> Response {
    if session.get::<String>("token").is_some() {
        match auth(session.get::<String>("token").unwrap()) {
            Ok(_) => BackDashboardTemplate {
                name: "Dashboard".to_string(),
            }
            .into_response(),
            Err(_) => Redirect::to("/login").into_response(),
        }
    } else {
        Redirect::to("/login").into_response()
    }
}
