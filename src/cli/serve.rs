use axum::error_handling::HandleErrorLayer;
use axum::routing::{get, get_service, post};
use axum::Router;
use pulldown_cmark::Options;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

use crate::configuration::Config;
use crate::handlers;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub tera: Tera,
    pub mk_parser_options: Options,
}

pub async fn serve(port: Option<u16>, timeout: Option<u64>) {
    let middleware_stack = ServiceBuilder::new()
        .layer(CatchPanicLayer::custom(handlers::error::panic))
        .layer(HandleErrorLayer::new(handlers::error::error))
        .layer(CompressionLayer::new())
        .timeout(Duration::from_secs(timeout.unwrap_or(5)));

    let config_file_content: String =
        fs::read_to_string("./config.toml").expect("Should read file ./config.toml");
    let config: Config = Config::new(config_file_content.as_str());
    let templates: String = format!("./themes/{}/src/**/*.html", config.theme.clone());

    let state: Arc<AppState> = Arc::new(AppState {
        config: config.clone(),
        tera: Tera::new(templates.as_str()).unwrap(),
        mk_parser_options: get_markdown_parser_options(),
    });

    let services: Router = Router::new().nest_service(
        "/assets/",
        get_service(ServeDir::new(format!("./themes/{}/assets", config.theme))),
    );

    let routes: Router = Router::new()
        .route("/", get(handlers::home::show))
        .route("/error", get(handlers::error::show))
        .route("/sitemap.xml", get(handlers::sitemap::show))
        .route("/rss", get(handlers::rss::show))
        .route("/search", post(handlers::search::show))
        .route("/tags/:tag", get(handlers::tag::show))
        .route("/*path", get(handlers::page::show))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port.unwrap_or(3000)));
    let app = Router::new()
        .merge(services)
        .merge(routes)
        .layer(middleware_stack);

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_markdown_parser_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options
}
