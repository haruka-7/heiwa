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

mod handlers;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an heiwa website project
    Init,
    /// Launch the webserver on localhost to access the website
    Serve,
}

#[derive(Clone)]
pub struct AppState {
    pub config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            println!("init")
        }
        Some(Commands::Serve) => serve().await,
        None => {}
    }
}

async fn serve() {
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
