use axum::error_handling::HandleErrorLayer;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::routing::{get, get_service};
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
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
        .layer(CatchPanicLayer::custom(
            handlers::front::error_handler::panic,
        ))
        .layer(HandleErrorLayer::new(
            handlers::front::error_handler::error,
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
        .route("/", get(handlers::front::home_handler::show));
}

fn routes_statics() -> Router {
    Router::new().nest_service(
        "/statics/",
        get_service(ServeDir::new("./templates/statics")),
    )
}
