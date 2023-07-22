use crate::templates::BackDashboardTemplate;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::WritableSession;

pub async fn show(mut session: WritableSession) -> Response {
    if session.get::<String>("author_name").is_some() {
        // TODO do not work
        session.expire_in(std::time::Duration::from_secs(15778800));
        BackDashboardTemplate {
            name: session.get("author_name").unwrap(),
        }
        .into_response()
    } else {
        // TODO add 403 status code
        Redirect::to("/login").into_response()
    }
}
