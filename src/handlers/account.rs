use crate::entities::authors::NewAuthor;
use crate::handlers::api::authors::FormLoginAuthor;
use crate::services::authors::{auth_api_call, create_author_api_call, find_author_api_call};
use crate::services::session::{session_insert_alert, session_remove_alert};
use crate::templates::{LoginTemplate, RegisterTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use std::string::ToString;

/// TODO use a toml file
const LOGIN_ALERT: &str = "Login et/ou mot de passe incorrect.";
const REGISTER_ALERT: &str = "Erreur lors de la création du compte";

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
    match find_author_api_call(form.name.clone()).await {
        Ok(_) => do_login(session, form).await,
        Err(_) => {
            session_insert_alert(&mut session, LOGIN_ALERT);
            Redirect::to("/login").into_response()
        }
    }
}

pub async fn logout_action(mut session: WritableSession) -> Redirect {
    session.destroy();
    Redirect::to("/")
}

pub async fn register(mut session: WritableSession) -> RegisterTemplate {
    let alert_message: String = session.get("alert").unwrap_or("".to_string());
    session_remove_alert(&mut session);
    RegisterTemplate {
        alert: alert_message,
    }
}

pub async fn register_action(
    mut session: WritableSession,
    Form(form): Form<NewAuthor>,
) -> Response {
    let password: String = form.password.clone();
    let name: String = form.name.clone();
    match create_author_api_call(form).await {
        Ok(_) => {
            let auth_author: FormLoginAuthor = FormLoginAuthor { name, password };
            do_login(session, auth_author).await
        }
        Err(()) => {
            session_insert_alert(&mut session, REGISTER_ALERT);
            Redirect::to("/register").into_response()
        }
    }
}

async fn do_login(mut session: WritableSession, form_login_author: FormLoginAuthor) -> Response {
    match auth_api_call(form_login_author).await {
        Ok(auth_author) => {
            session.insert("author_id", &auth_author.id).unwrap_or(());
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
