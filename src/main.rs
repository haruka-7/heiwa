use crate::configuration::{get_toml_content, Config};
use axum::{routing::get, routing::post, Router};
use std::{env, net::SocketAddr};

mod configuration;
mod handler;

#[tokio::main]
async fn main() {
    let config: Config = Config::new(get_toml_content(
        "/etc/heiwa/heiwa-authors.toml".to_string(),
    ));
    env::set_var("RUST_LOG", config.log_level);

    tracing_subscriber::fmt::init();

    let app = Router::new().merge(routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn routes() -> Router {
    Router::new()
        .route("/authors/create", post(handler::create))
        .route("/authors/:id", get(handler::author))
}
