use crate::services::config::Config;
use crate::services::database::connection_pool;
use axum::error_handling::HandleErrorLayer;
use axum::routing::{delete, get, get_service, patch, post};
use axum::Router;
use axum_sessions::{async_session, SessionLayer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use rand::RngCore;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

mod entities;
mod handlers;
mod middlewares;
mod schema;
mod services;
mod templates;

lazy_static! {
    static ref CONFIG: Config = Config::new();
}

#[derive(Clone)]
pub struct AppState {
    pub db_connection: Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let routes: Router = Router::new()
        .merge(routes_front())
        .nest("/dashboard", routes_dashboard())
        .nest("/api", routes_api())
        .fallback_service(routes_statics());

    let (app, addr) = init_server(routes);

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_server(routes: Router) -> (Router, SocketAddr) {
    tracing_subscriber::fmt::init();

    // Session secret must be at least 64 bytes
    let mut secret = [0u8; 128];
    rand::thread_rng().fill_bytes(&mut secret);

    let middleware_stack = ServiceBuilder::new()
        .layer(CatchPanicLayer::custom(handlers::error::panic))
        .layer(HandleErrorLayer::new(handlers::error::error))
        .layer(SessionLayer::new(
            async_session::MemoryStore::new(),
            &secret,
        ))
        .layer(CompressionLayer::new())
        .timeout(Duration::from_secs(CONFIG.server_timeout));

    let app = Router::new().merge(routes).layer(middleware_stack);
    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.server_port));
    (app, addr)
}

fn routes_dashboard() -> Router {
    Router::new()
        .route("/", get(handlers::backoffice::dashboard::show))
        .route("/articles", get(handlers::backoffice::articles::list))
        .route(
            "/article",
            get(handlers::backoffice::articles::new)
                .post(handlers::backoffice::articles::new_action),
        )
}

fn routes_front() -> Router {
    Router::new()
        .route("/", get(handlers::home::show))
        .route(
            "/login",
            get(handlers::account::login).post(handlers::account::login_action),
        )
        .route("/logout", get(handlers::account::logout_action))
        .route(
            "/register",
            get(handlers::account::register).post(handlers::account::register_action),
        )
        .route("/error-page", get(handlers::error::show))
}

fn routes_statics() -> Router {
    Router::new().nest_service(
        "/statics/",
        get_service(ServeDir::new("./templates/statics")),
    )
}

fn routes_api() -> Router {
    let state: Arc<AppState> = Arc::new(AppState {
        db_connection: connection_pool(),
    });

    Router::new()
        // Protected API routes with bearer jwt token
        .route("/authors/update", patch(handlers::api::authors::update))
        .route(
            "/authors/delete/:id",
            delete(handlers::api::authors::delete),
        )
        .route("/links/create", post(handlers::api::links::create))
        .route("/links/update", patch(handlers::api::links::update))
        .route("/links/delete/:id", delete(handlers::api::links::delete))
        .route("/articles/create", post(handlers::api::articles::create))
        // TODO add search and update articles route
        .route(
            "/articles/delete/:id",
            delete(handlers::api::articles::delete),
        )
        .route(
            "/tags/create/:article_id",
            post(handlers::api::tags::create),
        )
        .route(
            "/tags/delete/:article_id/:tag_id",
            delete(handlers::api::tags::delete),
        )


        // Unprotected API routes
        .route("/authors/login", post(handlers::api::authors::login))
        .route("/authors/create", post(handlers::api::authors::create))
        //TODO SECURITY this return a full Author with hashed password
        .route("/authors/get/:name", get(handlers::api::authors::get))
        .route("/links/get/:author_name", get(handlers::api::links::get))
        .route(
            "/articles/get/:permalink",
            get(handlers::api::articles::get),
        )
        .route("/articles/tag/:tag_id", get(handlers::api::articles::tag))
        .route(
            "/articles/author/:author_id",
            get(handlers::api::articles::author),
        )
        .route(
            "/tags/get/:article_permalink",
            get(handlers::api::tags::get),
        )
        .with_state(state)
}
