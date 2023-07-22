use crate::services::session::is_author_logged;
use crate::templates::{BackArticleNewTemplate, BackArticlesListTemplate};
use askama_axum::Response;
use axum::response::{IntoResponse, Redirect};
use axum_sessions::extractors::WritableSession;

pub async fn list(mut session: WritableSession) -> Response {
    if is_author_logged(&mut session) {
        BackArticlesListTemplate {
            alert: "list".to_string(),
        }
        .into_response()
    } else {
        // TODO add 403 status code
        Redirect::to("/login").into_response()
    }
}

pub async fn new(mut session: WritableSession) -> Response {
    if is_author_logged(&mut session) {
        BackArticleNewTemplate {
            alert: "new".to_string(),
        }
        .into_response()
    } else {
        // TODO add 403 status code
        Redirect::to("/login").into_response()
    }
}

pub async fn new_action(mut session: WritableSession) -> Redirect {
    if is_author_logged(&mut session) {
        Redirect::to("/dashboard/articles")
    } else {
        // TODO add 403 status code
        Redirect::to("/login")
    }
}
