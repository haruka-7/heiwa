use axum::{routing::delete, routing::get, routing::post, Router};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

mod entities;
mod handlers;
mod schema;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let server_timeout: u64 = env::var("SERVER_TIMEOUT")
        .unwrap_or("5".to_string())
        .parse::<i64>()
        .expect("SERVER_PORT environment variable should be parsed correctly")
        as u64;

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(HandleErrorLayer::new(handlers::errors::error))
        .timeout(Duration::from_secs(server_timeout));

    let app = Router::new()
        .merge(routes())
        .layer(middleware_stack);

    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or("3000".to_string())
        .parse::<i16>()
        .expect("SERVER_PORT environment variable should be parsed correctly")
        as u16;

    let addr = SocketAddr::from(([127, 0, 0, 1], server_port));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn routes() -> Router {
    Router::new()
        .route("/authors/create", post(handlers::authors::create))
        .route("/authors/delete/:id", delete(handlers::authors::delete))
        .route("/authors/get/:name", get(handlers::authors::get))
        .route("/authors/login", post(handlers::authors::login))
        .route("/links/create", post(handlers::links::create))
        .route("/links/delete/:id", delete(handlers::links::delete))
        .route("/links/get/:author_name", get(handlers::links::get))
}