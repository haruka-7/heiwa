use crate::services::authors::is_author_logged;
use crate::services::session::session_remove_alert;
use crate::templates::{BackArticleNewTemplate, BackArticlesListTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::WritableSession;

pub async fn list(session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => BackArticlesListTemplate {
            alert: "list".to_string(),
        }
        .into_response(),
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn new(mut session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            let alert_message: String = session.get("alert").unwrap_or("".to_string());
            session_remove_alert(&mut session);
            BackArticleNewTemplate {
                alert: alert_message,
            }
            .into_response()
        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn new_action() -> Redirect {
    Redirect::to("/dashboard/articles")
}
