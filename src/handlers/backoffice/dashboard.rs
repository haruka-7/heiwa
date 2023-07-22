use crate::services::session::is_author_logged;
use crate::templates::BackDashboardTemplate;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::WritableSession;

pub async fn show(mut session: WritableSession) -> Response {
    if is_author_logged(&mut session) {
        BackDashboardTemplate {
            name: session.get("author_name").unwrap(),
        }
        .into_response()
    } else {
        // TODO add 403 status code
        Redirect::to("/login").into_response()
    }
}
