use axum::response::{IntoResponse, Redirect, Response};
use axum::{
    http::Request,
    middleware::Next,
};
use axum_extra::extract::cookie::CookieJar;
use axum_sessions::extractors::WritableSession;
use std::time::Duration;
use time::OffsetDateTime;

pub async fn auth_session_required<B>(
    cookie_jar: CookieJar,
    mut session: WritableSession,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    tracing::error!("session author name {}", session.get::<String>("author_name").unwrap());
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
        tracing::error!("session author not found");
        Redirect::to("/login").into_response()
    }
}
