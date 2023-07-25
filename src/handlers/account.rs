use crate::entities::authors::{Author, LoginAuthor, LoginAuthorPassword, NewAuthor};
use crate::services::author::author_sign_in;
use crate::services::session::{session_insert_alert, session_remove_alert};
use crate::templates::{LoginTemplate, RegisterTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use diesel::QueryResult;
use std::string::ToString;
use crate::entities::roles::Roles;

// TODO duplicated const
const LOGIN_ALERT: &str = "Login et/ou mot de passe incorrect.";

pub async fn login(session: WritableSession) -> Response {
    if session.get::<String>("author_name").is_some() {
        return Redirect::to("/dashboard").into_response();
    }
    let alert_message: String = session.get("alert").unwrap_or("".to_string());
    session_remove_alert(session);
    LoginTemplate {
        alert: alert_message,
    }
    .into_response()
}

pub async fn login_action(session: WritableSession, Form(form): Form<LoginAuthor>) -> Redirect {
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(form.name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                session_insert_alert(session, LOGIN_ALERT);
                Redirect::to("/login")
            } else {
                let author: &LoginAuthorPassword = author.first().unwrap();
                author_sign_in(
                    session,
                    form.password,
                    &author.name,
                    &author.password,
                    &author.role.clone().unwrap_or(Roles::AUTHOR.to_string()),
                )
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
    Redirect::to("/")
}

pub async fn register() -> RegisterTemplate {
    RegisterTemplate {
        alert: "".to_string(),
    }
}

pub async fn register_action(session: WritableSession, Form(form): Form<NewAuthor>) -> Redirect {
    let form_password = form.password.clone();
    let author_result: QueryResult<LoginAuthorPassword> = Author::create(form);
    match author_result {
        Ok(author) => author_sign_in(
            session,
            (form_password).parse().unwrap(),
            &author.name,
            &author.password,
            &author.role.unwrap_or(Roles::AUTHOR.to_string()),
        ),
        Err(e) => {
            tracing::error!("{}", e);
            session_insert_alert(session, LOGIN_ALERT);
            Redirect::to("/login")
        }
    }
}
