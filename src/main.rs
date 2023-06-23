use axum::{routing::get, routing::post, routing::delete, Router};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

mod entities;
mod handlers;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new().merge(routes());

    let server_port: u16 = env::var("SERVER_PORT")
        .expect("SERVER_PORT environment variable should exist")
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
        .route("/authors", get(handlers::authors))
        .route("/authors/create", post(handlers::create))
        .route("/authors/:name", get(handlers::author))
        .route("/authors/delete/:id", delete(handlers::delete))
}
