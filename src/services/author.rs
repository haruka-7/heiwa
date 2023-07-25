use crate::entities::authors::verify_password;
use crate::services::session::session_insert_alert;
use axum::response::Redirect;
use axum_sessions::extractors::WritableSession;

// TODO duplicated const
const LOGIN_ALERT: &str = "Login et/ou mot de passe incorrect.";

pub fn author_login(
    mut session: WritableSession,
    form_password: String,
    author_name: &String,
    author_password: &str,
    author_role: &String,
) -> Redirect {
    if verify_password(form_password, author_password) {
        session
            .insert("author_name", author_name)
            .expect("Should insert author name in session");
        session
            .insert("author_role", author_role)
            .expect("Should insert author role in session");
        Redirect::to("/dashboard")
    } else {
        session_insert_alert(session, LOGIN_ALERT);
        Redirect::to("/login")
    }
}
