use crate::entities::authors::{Author, NewAuthor};
use crate::handlers::api::authors::FormLoginAuthor;
use crate::services::authors::auth;
use crate::services::session::{session_insert_alert, session_remove_alert};
use crate::templates::{LoginTemplate, RegisterTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use diesel::QueryResult;
use std::string::ToString;

// TODO duplicated const
const LOGIN_ALERT: &str = "Login et/ou mot de passe incorrect.";

pub async fn login(mut session: WritableSession) -> Response {
    if session.get::<String>("token").is_some() {
        return Redirect::to("/dashboard").into_response();
    }
    let alert_message: String = session.get("alert").unwrap_or("".to_string());
    session_remove_alert(&mut session);
    LoginTemplate {
        alert: alert_message,
    }
    .into_response()
}

pub async fn login_action(
    mut session: WritableSession,
    Form(form): Form<FormLoginAuthor>,
) -> Response {
    let author_result = Author::find_by_name(form.name.clone());
    match author_result {
        Ok(authors) => {
            if authors.is_empty() {
                session_insert_alert(&mut session, LOGIN_ALERT);
                Redirect::to("/login").into_response()
            } else {
                do_login(session, form).await
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            session_insert_alert(&mut session, LOGIN_ALERT);
            Redirect::to("/login").into_response()
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

pub async fn register_action(
    mut session: WritableSession,
    Form(form): Form<NewAuthor>,
) -> Response {
    let form_password: String = form.password.clone();
    let author_result: QueryResult<Author> = Author::create(form);
    match author_result {
        Ok(author) => {
            let auth_author: FormLoginAuthor = FormLoginAuthor {
                name: author.name,
                password: form_password,
            };
            do_login(session, auth_author).await
        }
        Err(e) => {
            tracing::error!("{}", e);
            session_insert_alert(&mut session, LOGIN_ALERT);
            Redirect::to("/login").into_response()
        }
    }
}

async fn do_login(mut session: WritableSession, form_login_author: FormLoginAuthor) -> Response {
    match auth(form_login_author).await {
        Ok(auth_author) => {
            session.insert("token", &auth_author.token).unwrap_or(());
            session.insert("role", &auth_author.role).unwrap_or(());
            Redirect::to("/dashboard").into_response()
        }
        Err(_) => {
            session_insert_alert(&mut session, LOGIN_ALERT);
            Redirect::to("/login").into_response()
        }
    }
}
