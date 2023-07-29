use crate::entities::authors::{Author, LoginAuthor, LoginAuthorPassword, NewAuthor};
use crate::services::session::{session_insert_alert, session_remove_alert};
use crate::templates::{LoginTemplate, RegisterTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use diesel::QueryResult;
use std::string::ToString;
use crate::services::authors::auth;

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

pub async fn login_action(mut session: WritableSession, Form(form): Form<LoginAuthor>) -> Response {
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(form.name.clone());
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                session_insert_alert(&mut session, LOGIN_ALERT);
                Redirect::to("/login").into_response()
            } else {
                match auth(form).await {
                    Ok(token) => {
                        session.insert("token", &token).unwrap_or(());
                        Redirect::to("/dashboard").into_response()
                    }
                    Err(_) => {
                        session_insert_alert(&mut session, LOGIN_ALERT);
                        Redirect::to("/login").into_response()
                    }
                }
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

pub async fn register_action(mut session: WritableSession, Form(form): Form<NewAuthor>) -> Response {
    let author_result: QueryResult<LoginAuthorPassword> = Author::create(form);
    match author_result {
        Ok(author) => {
            let login_author: LoginAuthor = LoginAuthor {
                name: author.name,
                password: author.password,
            };
            login_action(session, Form(login_author)).await
        }
        Err(e) => {
            tracing::error!("{}", e);
            session_insert_alert(&mut session, LOGIN_ALERT);
            Redirect::to("/login").into_response()
        }
    }
}
