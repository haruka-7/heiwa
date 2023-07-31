use crate::services::authors::is_author_logged;
use crate::templates::BackDashboardTemplate;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::WritableSession;

pub async fn show(session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => BackDashboardTemplate {
            name: "Dashboard".to_string(),
        }
        .into_response(),
        Err(_) => Redirect::to("/login").into_response(),
    }
}
