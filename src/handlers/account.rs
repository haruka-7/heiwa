use crate::entities::authors::{
    verify_password, Author, AuthorAccessToken, LoginAuthor, LoginAuthorPassword, NewAuthor,
};
use crate::services::session::{session_insert_alert, session_insert_token, session_remove_alert};
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

pub async fn login_action(mut session: WritableSession, Form(form): Form<LoginAuthor>) -> Response {
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(form.name.clone());
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                session_insert_alert(&mut session, LOGIN_ALERT);
                Redirect::to("/login").into_response()
            } else {
                let author: &LoginAuthorPassword = author.first().unwrap();
                match verify_password(&form.password, &author.password) {
                    Ok(_) => {
                        let client = reqwest::Client::new();
                        let response = client
                            .post("http://localhost:3000/api/authors/login")
                            .json(&form)
                            .send()
                            .await
                            .unwrap()
                            .json::<AuthorAccessToken>()
                            .await
                            .unwrap();
                        if !response.access_token.is_empty() {
                            session_insert_token(&mut session, &response.access_token);
                            Redirect::to("/dashboard").into_response()
                        } else {
                            tracing::error!(
                                "Error while geting JWT token for author : {}",
                                author.name
                            );
                            Redirect::to("/error-page").into_response()
                        }
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

pub async fn register_action(
    mut session: WritableSession,
    Form(form): Form<NewAuthor>,
) -> Redirect {
    let form_password = form.password.clone();
    let author_result: QueryResult<LoginAuthorPassword> = Author::create(form);
    match author_result {
        Ok(author) => match verify_password(&(form_password).parse().unwrap(), &author.password) {
            Ok(_) => Redirect::to("/dashboard"),
            Err(_) => Redirect::to("/login"),
        },
        Err(e) => {
            tracing::error!("{}", e);
            session_insert_alert(&mut session, LOGIN_ALERT);
            Redirect::to("/login")
        }
    }
}
