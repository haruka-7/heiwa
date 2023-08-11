use crate::services::config_service::Config;
use crate::services::database_service::connection_pool;
use axum::error_handling::HandleErrorLayer;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
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
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod entities;
mod handlers;
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
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(CatchPanicLayer::custom(
            handlers::front::error_handler::panic,
        ))
        .layer(HandleErrorLayer::new(
            handlers::front::error_handler::error,
        ))
        .layer(
            SessionLayer::new(async_session::MemoryStore::new(), &secret)
                // expiration set to 6 months matching JWT expiration
                .with_session_ttl(Some(Duration::from_secs(15778800))),
        )
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
        .route("/", get(handlers::front::home_handler::show))
        .route(
            "/login",
            get(handlers::front::account_handler::login)
                .post(handlers::front::account_handler::login_action),
        )
        .route(
            "/logout",
            get(handlers::front::account_handler::logout_action),
        )
        .route(
            "/register",
            get(handlers::front::account_handler::register)
                .post(handlers::front::account_handler::register_action),
        )
        .route("/error-page", get(handlers::front::error_handler::show))
        .nest(
            "/api",
            Router::new()
                .route(
                    "/token/:id",
                    get(handlers::api::tokens_handler::token_verify),
                )
                // Authors - TODO SECURITY this return a full Author with hashed password
                .route(
                    "/authors/find/:name",
                    get(handlers::api::authors_api_handler::find),
                )
                .route(
                    "/authors/get/:id",
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
                    "/articles/all",
                    get(handlers::api::articles_api_handler::get_all),
                )
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
                .route(
                    "/articles/edit/:permalink",
                    put(handlers::api::articles_api_handler::update),
                )
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
                )
                .route(
                    "/article/update",
                    post(handlers::dashboard::articles_handler::edit_action),
                )
                .route(
                    "/profile",
                    get(handlers::dashboard::profile_handler::edit)
                        .post(handlers::dashboard::profile_handler::edit_action),
                ),
        )
        .route("/:permalink", get(handlers::front::article_handler::show))
        .with_state(state)
}

fn routes_statics() -> Router {
    Router::new().nest_service(
        "/statics/",
        get_service(ServeDir::new("./templates/statics")),
    )
}
