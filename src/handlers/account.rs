use axum::http::Request;
use crate::entities::authors::{verify_password, LoginAuthor, LoginAuthorPassword, Author};
use crate::templates::{DashboardTemplate, LoginTemplate, RegisterTemplate};
use axum::response::Redirect;
use axum::Json;
use axum_sessions::SessionHandle;
use diesel::QueryResult;
use hyper::Body;

pub async fn login() -> LoginTemplate {
    LoginTemplate {
        alert: "".to_string(),
    }
}

pub async fn login_action(request: Request<Body>, Json(payload): Json<LoginAuthor>) -> Redirect {
    let author_result: QueryResult<Vec<Author>> =
        Author::find_by_name(payload.name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                Redirect::permanent("/login")
            } else {
                let author: &Author = author.first().unwrap();
                if verify_password(payload.password, &author.password) {
                    let session_handle = request.extensions().get::<SessionHandle>().unwrap();
                    let mut session = session_handle.write().await;
                    session.insert("author_name", &author.name).expect("Should insert author name in session");
                    session.insert("author_role", &author.role).expect("Should insert author role in session");
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

pub async fn logout_action(request: Request<Body>) -> Redirect {
    let session_handle = request.extensions().get::<SessionHandle>().unwrap();
    let mut session = session_handle.read().await;
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

pub async fn dashboard() -> DashboardTemplate {
    DashboardTemplate {
        name: "".to_string(),
    }
}
