use crate::services::authors_service::is_author_logged;
use crate::templates::dashboard_templates::DashboardHomeTemplate;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::WritableSession;

pub async fn show(session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => DashboardHomeTemplate {
            name: "Dashboard".to_string(),
        }
        .into_response(),
        Err(_) => Redirect::to("/login").into_response(),
    }
}