use crate::templates::BackDashboardTemplate;
use axum_sessions::extractors::WritableSession;

pub async fn show(session: WritableSession) -> BackDashboardTemplate {
    BackDashboardTemplate {
        name: session.get("author_name").unwrap(),
    }
}
