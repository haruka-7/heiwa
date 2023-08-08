use crate::services::config_service::Config;
use crate::services::database_service::connection_pool;
use axum::error_handling::HandleErrorLayer;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::routing::{delete, get, get_service, patch, post, put};
use axum::Router;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
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

    let middleware_stack = ServiceBuilder::new()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(CatchPanicLayer::custom(handlers::error_handler::panic))
        .layer(HandleErrorLayer::new(handlers::error_handler::error))
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
        .route("/token/:id", get(handlers::tokens_handler::token_verify))
        // Authors - TODO SECURITY this return a full Author with hashed password
        .route(
            "/authors/find/:name",
            get(handlers::authors_api_handler::find),
        )
        .route("/authors/get/:id", get(handlers::authors_api_handler::get))
        .route("/authors/login", post(handlers::authors_api_handler::login))
        .route(
            "/authors/create",
            post(handlers::authors_api_handler::create),
        )
        .route(
            "/authors/update",
            patch(handlers::authors_api_handler::update),
        )
        .route(
            "/authors/delete/:id",
            delete(handlers::authors_api_handler::delete),
        )
        // Links
        .route(
            "/links/get/:author_name",
            get(handlers::links_api_handler::get),
        )
        .route("/links/create", post(handlers::links_api_handler::create))
        .route("/links/update", patch(handlers::links_api_handler::update))
        .route(
            "/links/delete/:id",
            delete(handlers::links_api_handler::delete),
        )
        // Articles - TODO add search and update articles route
        .route(
            "/articles/all",
            get(handlers::articles_api_handler::get_all),
        )
        .route(
            "/articles/get/:permalink",
            get(handlers::articles_api_handler::get),
        )
        .route(
            "/articles/author/:author_id",
            get(handlers::articles_api_handler::author),
        )
        .route(
            "/articles/tag/:tag_id",
            get(handlers::articles_api_handler::tag),
        )
        .route(
            "/articles/create",
            post(handlers::articles_api_handler::create),
        )
        .route(
            "/articles/edit/:permalink",
            put(handlers::articles_api_handler::update),
        )
        .route(
            "/articles/delete/:id",
            delete(handlers::articles_api_handler::delete),
        )
        // Tags
        .route(
            "/tags/get/:article_permalink",
            get(handlers::tags_api_handler::get),
        )
        .route(
            "/tags/create/:article_id",
            post(handlers::tags_api_handler::create),
        )
        .route(
            "/tags/delete/:article_id/:tag_id",
            delete(handlers::tags_api_handler::delete),
        )
        .with_state(state)
}

fn routes_statics() -> Router {
    Router::new().nest_service(
        "/statics/",
        get_service(ServeDir::new("./templates/statics")),
    )
}
