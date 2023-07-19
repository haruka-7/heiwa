use axum::Json;
use diesel::QueryResult;
use crate::entities::authors::{LoginAuthor, LoginAuthorPassword, verify_password};
use crate::templates::{LoginTemplate, RegisterTemplate};

pub async fn login() -> LoginTemplate {
    LoginTemplate {
        alert: "".to_string(),
    }
}

pub async fn login_action(Json(payload): Json<LoginAuthor>) {
    let author_result: QueryResult<Vec<LoginAuthorPassword>> =
        LoginAuthorPassword::find_by_name_for_login(payload.name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                // TODO redirect login handler
            } else {
                let author: &LoginAuthorPassword = author.first().unwrap();
                if verify_password(payload.password, &author.password) {
                    // TODO redirect dashboard handler
                } else {
                    // TODO redirect login handler
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            // TODO redirect login handler
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
