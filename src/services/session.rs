use axum_sessions::extractors::WritableSession;

pub fn is_author_logged(session: &mut WritableSession) -> bool {
    if session.get::<String>("author_name").is_some() {
        // TODO do not work
        session.expire_in(std::time::Duration::from_secs(15778800));
        true
    } else {
        false
    }
}

pub fn session_insert_alert(mut session: WritableSession, alert_message: &str) {
    session.insert("alert", alert_message).unwrap_or(());
}

pub fn session_remove_alert(mut session: WritableSession) {
    session.remove("alert");
}
