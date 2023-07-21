use crate::entities::authors::{verify_password, LoginAuthor, LoginAuthorPassword};
use crate::services::session::{session_insert_alert, session_remove_alert};
use crate::templates::{DashboardTemplate, LoginTemplate, RegisterTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use diesel::QueryResult;
use std::string::ToString;
use axum::http::StatusCode;

const LOGIN_ALERT: &str = "Login et/ou mot de passe incorrect.";

pub async fn login(session: WritableSession) -> LoginTemplate {
    let alert_message: String = session.get("alert").unwrap_or("".to_string());
    session_remove_alert(session);
    LoginTemplate {
        alert: alert_message,
    }
}

pub async fn login_action(mut session: WritableSession, Form(form): Form<LoginAuthor>) -> Redirect {
    if session.get("author_name").is_some() {
        return Redirect::to("/login")
    }
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(form.name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                session_insert_alert(session, LOGIN_ALERT);
                Redirect::to("/login")
            } else {
                let author: &LoginAuthorPassword = author.first().unwrap();
                if verify_password(form.password, &author.password) {
                    // Session duration set to 6 mouths
                    session
                        .insert("author_name", &author.name)
                        .expect("Should insert author name in session");
                    session
                        .insert("author_role", author.role.clone().unwrap())
                        .expect("Should insert author role in session");
                    Redirect::to("/dashboard")
                } else {
                    session_insert_alert(session, LOGIN_ALERT);
                    Redirect::to("/login")
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            session_insert_alert(session, LOGIN_ALERT);
            Redirect::to("/login")
        }
    }
}

pub async fn logout_action(mut session: WritableSession) -> Redirect {
    session.destroy();
    Redirect::permanent("/")
}

pub async fn register() -> RegisterTemplate {
    RegisterTemplate {
        name: "register".to_string(),
    }
}

pub async fn register_action() -> RegisterTemplate {
    RegisterTemplate {
        name: "register action".to_string(),
    }
}

pub async fn dashboard(mut session: WritableSession) -> Response {
    if session.get("author_name").is_some() {
        session.expire_in(std::time::Duration::from_secs(15778800));
        DashboardTemplate { name: session.get("author_name").unwrap() }.into_response()
    } else {
        (StatusCode::FORBIDDEN, Redirect::to("/login")).into_response()
    }
}
