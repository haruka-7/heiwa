use crate::services::jwt::verify;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{http::Request, middleware::Next};
use axum_auth::AuthBearer;
use axum_extra::extract::cookie::CookieJar;
use axum_sessions::extractors::WritableSession;
use std::time::Duration;
use time::OffsetDateTime;

pub async fn authorization_bearer_required<B>(
    token: AuthBearer,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    match verify(token.0.as_str()) {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn auth_session_required<B>(
    cookie_jar: CookieJar,
    mut session: WritableSession,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if session.get::<String>("author_name").is_some() {
        tracing::error!("session author FOUND");
        // 6 months expiration
        let expire_from_secs: Duration = Duration::from_secs(15778800);
        let expire = OffsetDateTime::now_utc() + expire_from_secs;
        session.expire_in(expire_from_secs);
        let mut cookie = cookie_jar.get("sid").unwrap().to_owned();
        cookie.set_expires(expire);
        next.run(request).await
    } else {
        tracing::error!("session author NOT FOUND");
        Redirect::to("/login").into_response()
    }
}
