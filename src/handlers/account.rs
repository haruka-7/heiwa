use crate::entities::authors::{verify_password, LoginAuthor, LoginAuthorPassword};
use crate::templates::{LoginTemplate, RegisterTemplate};
use axum::response::Redirect;
use axum::Json;
use diesel::QueryResult;

pub async fn login() -> LoginTemplate {
    LoginTemplate {
        alert: "".to_string(),
    }
}

pub async fn login_action(Json(payload): Json<LoginAuthor>) -> Redirect {
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(payload.name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                Redirect::permanent("/login")
            } else {
                let author: &LoginAuthorPassword = author.first().unwrap();
                if verify_password(payload.password, &author.password) {
                    Redirect::permanent("/dashboard")
                } else {
                    Redirect::permanent("/login")
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Redirect::permanent("/login")
        }
    }
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
