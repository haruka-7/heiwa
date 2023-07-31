use crate::services::session::session_remove_alert;
use crate::templates::{BackArticleNewTemplate, BackArticlesListTemplate};
use axum::response::Redirect;
use axum_sessions::extractors::WritableSession;

pub async fn list() -> BackArticlesListTemplate {
    BackArticlesListTemplate {
        alert: "list".to_string(),
    }
}

pub async fn new(mut session: WritableSession) -> BackArticleNewTemplate {
    let alert_message: String = session.get("alert").unwrap_or("".to_string());
    session_remove_alert(&mut session);
    BackArticleNewTemplate {
        alert: alert_message,
    }
}

pub async fn new_action() -> Redirect {
    Redirect::to("/dashboard/articles")
}
