use axum_sessions::extractors::WritableSession;

pub fn session_insert_alert(session: &mut WritableSession, alert_message: &str) {
    session.insert("alert", alert_message).unwrap_or(());
}

pub fn session_remove_alert(session: &mut WritableSession) {
    session.remove("alert");
}

pub fn session_insert_token(session: &mut WritableSession, token: &str) {
    session.insert("token", token).unwrap_or(());
}
