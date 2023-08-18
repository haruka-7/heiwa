use axum::error_handling::HandleErrorLayer;
use axum::routing::{get, get_service};
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub config: String,
}

pub async fn serve() {
    let middleware_stack = ServiceBuilder::new()
        .layer(CatchPanicLayer::custom(handlers::error::panic))
        .layer(HandleErrorLayer::new(handlers::error::error))
        .layer(CompressionLayer::new())
        .timeout(Duration::from_secs(5));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .merge(routes())
        .layer(middleware_stack)
        .fallback_service(routes_statics());

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn routes() -> Router {
    let state: Arc<AppState> = Arc::new(AppState {
        config: "config".to_string(),
    });
    Router::new()
        .route("/", get(handlers::home::show))
        .route("/error", get(handlers::error::show))
        .with_state(state)
}

fn routes_statics() -> Router {
    Router::new()
        .nest_service(
            "/statics/",
            get_service(ServeDir::new("./themes/default/statics")),
        )
        .nest_service("/medias/", get_service(ServeDir::new("./medias")))
}
