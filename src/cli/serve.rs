use axum::error_handling::HandleErrorLayer;
use axum::routing::{get, get_service};
use axum::Router;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

use crate::configuration::Config;
use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}

pub async fn serve(port: Option<u16>, timeout: Option<u64>) {
    let middleware_stack = ServiceBuilder::new()
        .layer(CatchPanicLayer::custom(handlers::error::panic))
        .layer(HandleErrorLayer::new(handlers::error::error))
        .layer(CompressionLayer::new())
        .timeout(Duration::from_secs(timeout.unwrap_or(5)));

    let config_file_content: String =
        fs::read_to_string("./config.toml").expect("Should read file ./config.toml");
    let state: Arc<AppState> = Arc::new(AppState {
        config: Config::new(config_file_content.as_str()),
    });

    let routes: Router = Router::new()
        .route("/", get(handlers::home::show))
        .route("/error", get(handlers::error::show))
        .with_state(state);

    let services: Router = Router::new()
        .nest_service(
            "/statics/",
            get_service(ServeDir::new("./themes/default/statics")),
        )
        .nest_service("/medias/", get_service(ServeDir::new("./medias")));

    let addr = SocketAddr::from(([127, 0, 0, 1], port.unwrap_or(3000)));
    let app = Router::new()
        .merge(routes)
        .layer(middleware_stack)
        .fallback_service(services);

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
