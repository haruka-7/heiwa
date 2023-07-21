use axum_sessions::extractors::WritableSession;

pub fn session_insert_alert(mut session: WritableSession, alert_message: &str) {
    session.insert("alert", alert_message).unwrap_or(());
}

pub fn session_remove_alert(mut session: WritableSession) {
    session.remove("alert");
}
