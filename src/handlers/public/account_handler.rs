use crate::entities::authors_entity::NewAuthor;
use crate::handlers::api::authors_api_handler::FormLoginAuthor;
use crate::services::authors_service::{auth_author, create_author};
use crate::services::session_service::{session_insert_alert, session_remove_alert};
use crate::templates::site_templates::{LoginTemplate, RegisterTemplate};
use crate::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use std::string::ToString;
use std::sync::Arc;

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
    State(state): State<Arc<AppState>>,
    session: WritableSession,
    Form(form): Form<FormLoginAuthor>,
) -> Response {
    do_login(&state, session, form)
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
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Form(form): Form<NewAuthor>,
) -> Response {
    let password: String = form.password.clone();
    let name: String = form.name.clone();
    match create_author(&state, form) {
        Ok(_) => {
            let auth_author: FormLoginAuthor = FormLoginAuthor { name, password };
            do_login(&state, session, auth_author)
        }
        Err(error_code) => {
            match error_code {
                None => session_insert_alert(&mut session, REGISTER_ALERT),
                Some(code) => session_insert_alert(&mut session, code.as_str()),
            }
            Redirect::to("/register").into_response()
        }
    }
}

fn do_login(
    state: &Arc<AppState>,
    mut session: WritableSession,
    form_login_author: FormLoginAuthor,
) -> Response {
    match auth_author(&state, form_login_author) {
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
