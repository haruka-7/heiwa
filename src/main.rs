use axum::{routing::delete, routing::get, routing::post, Router};
use heiwa_common::init_server;

mod entities;
mod handlers;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    let routes: Router = Router::new()
        .route("/authors/create", post(handlers::authors::create))
        // TODO add update author route
        .route("/authors/delete/:id", delete(handlers::authors::delete))
        .route("/authors/get/:name", get(handlers::authors::get))
        .route("/authors/login", post(handlers::authors::login))
        .route("/links/create", post(handlers::links::create))
        .route("/links/delete/:id", delete(handlers::links::delete))
        .route("/links/get/:author_name", get(handlers::links::get));

    let (app, addr) = init_server(routes);

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
