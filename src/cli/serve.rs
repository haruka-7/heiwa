use crate::configuration::Config;
use crate::entities::page::Page;
use crate::handlers;
use crate::utils::file::read_file;
use axum::error_handling::HandleErrorLayer;
use axum::routing::{get, get_service, post};
use axum::Router;
use glob::glob;
use pulldown_cmark::Options;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

#[derive(Debug, Clone)]
pub struct AppState {
    pub path: String,
    pub config: Config,
    pub tera: Tera,
    pub tags: Vec<String>,
    pub mk_parser_options: Options,
}

impl AppState {
    fn new(path: String) -> Self {
        let config_file_content: String =
            fs::read_to_string(&format!("{}/config.toml", path)).expect(&format!("Should read file {}/config.toml", path));
        let config: Config = Config::new(config_file_content.as_str());
        let templates: String = format!("{}/themes/{}/src/**/*.html", path, config.theme);
        let tags = get_tags(&path);

        AppState {
            path,
            config,
            tera: Tera::new(templates.as_str()).unwrap(),
            tags,
            mk_parser_options: get_markdown_parser_options(),
        }
    }
}

pub async fn serve(path: String, port: Option<u16>, timeout: Option<u64>) {
    let state: Arc<AppState> = Arc::new(AppState::new(path));
    let theme_path: String = format!("{}/themes/{}", state.path, state.config.theme);

    if state.config.theme.is_empty()
        || !Path::new(&theme_path).is_dir()
    {
        panic!(
            "No theme found {} please download a theme and verify config.toml",
            state.config.theme
        );
    }

    let middleware_stack = ServiceBuilder::new()
        .layer(CatchPanicLayer::custom(handlers::error::panic))
        .layer(HandleErrorLayer::new(handlers::error::error))
        .layer(CompressionLayer::new())
        .timeout(Duration::from_secs(timeout.unwrap_or(5)));

    let services: Router = Router::new().nest_service(
        "/assets/",
        get_service(ServeDir::new(format!(
            "{}/assets",
            theme_path
        ))),
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

fn get_tags(project_path: &str) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for entry in glob(&format!("{}/pages/**/*.md", project_path)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_path: String = path.into_os_string().into_string().unwrap();
                let file_content: String = read_file(&file_path);
                let url: String = file_path.replace("pages/", "").replace(".md", "");
                let page: Page = Page::new(url, file_content, get_markdown_parser_options());
                if page.published && !page.tags.is_empty() {
                    for tag in page.tags {
                        if !tags.contains(&tag) {
                            tags.push(tag)
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }
    tags
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
