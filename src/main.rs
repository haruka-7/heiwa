use crate::services::config_service::Config;
use crate::services::database_service::connection_pool;
use axum::error_handling::HandleErrorLayer;
use axum::routing::{delete, get, get_service, patch, post, put};
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
        .merge(routes())
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
        .layer(CatchPanicLayer::custom(
            handlers::frontend::error_handler::panic,
        ))
        .layer(HandleErrorLayer::new(
            handlers::frontend::error_handler::error,
        ))
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

fn routes() -> Router {
    let state: Arc<AppState> = Arc::new(AppState {
        db_connection: connection_pool(),
    });
    Router::new()
        .route("/", get(handlers::frontend::home_handler::show))
        .route(
            "/login",
            get(handlers::frontend::account_handler::login)
                .post(handlers::frontend::account_handler::login_action),
        )
        .route(
            "/logout",
            get(handlers::frontend::account_handler::logout_action),
        )
        .route(
            "/register",
            get(handlers::frontend::account_handler::register)
                .post(handlers::frontend::account_handler::register_action),
        )
        .route("/error-page", get(handlers::frontend::error_handler::show))
        .nest(
            "/api",
            Router::new()
                // Authors - TODO SECURITY this return a full Author with hashed password
                .route(
                    "/authors/get/:name",
                    get(handlers::api::authors_api_handler::get),
                )
                .route(
                    "/authors/login",
                    post(handlers::api::authors_api_handler::login),
                )
                .route(
                    "/authors/create",
                    post(handlers::api::authors_api_handler::create),
                )
                .route(
                    "/authors/update",
                    patch(handlers::api::authors_api_handler::update),
                )
                .route(
                    "/authors/delete/:id",
                    delete(handlers::api::authors_api_handler::delete),
                )
                // Links
                .route(
                    "/links/get/:author_name",
                    get(handlers::api::links_api_handler::get),
                )
                .route(
                    "/links/create",
                    post(handlers::api::links_api_handler::create),
                )
                .route(
                    "/links/update",
                    patch(handlers::api::links_api_handler::update),
                )
                .route(
                    "/links/delete/:id",
                    delete(handlers::api::links_api_handler::delete),
                )
                // Articles - TODO add search and update articles route
                .route(
                    "/articles/get/:permalink",
                    get(handlers::api::articles_api_handler::get),
                )
                .route(
                    "/articles/author/:author_id",
                    get(handlers::api::articles_api_handler::author),
                )
                .route(
                    "/articles/tag/:tag_id",
                    get(handlers::api::articles_api_handler::tag),
                )
                .route(
                    "/articles/create",
                    post(handlers::api::articles_api_handler::create),
                )
                /*.route(
                    "/articles/edit/:permalink",
                    put(handlers::api::articles_api_handler::update),
                )*/
                .route(
                    "/articles/delete/:id",
                    delete(handlers::api::articles_api_handler::delete),
                )
                // Tags
                .route(
                    "/tags/get/:article_permalink",
                    get(handlers::api::tags_api_handler::get),
                )
                .route(
                    "/tags/create/:article_id",
                    post(handlers::api::tags_api_handler::create),
                )
                .route(
                    "/tags/delete/:article_id/:tag_id",
                    delete(handlers::api::tags_api_handler::delete),
                ),
        )
        .nest(
            "/dashboard",
            Router::new()
                .route("/", get(handlers::dashboard::dashboard_handler::show))
                .route(
                    "/articles",
                    get(handlers::dashboard::articles_handler::list),
                )
                .route(
                    "/article",
                    get(handlers::dashboard::articles_handler::new)
                        .post(handlers::dashboard::articles_handler::new_action),
                )
                .route(
                    "/article/:permalink",
                    get(handlers::dashboard::articles_handler::edit),
                ),
        )
        .with_state(state)
}

fn routes_statics() -> Router {
    Router::new().nest_service(
        "/statics/",
        get_service(ServeDir::new("./templates/statics")),
    )
}
