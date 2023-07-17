use std::{env, net::SocketAddr};
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

mod entities;
mod handlers;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    let (app, addr) = init_server(routes());

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_server(routes: Router) -> (Router, SocketAddr) {
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

    let app = Router::new().merge(routes).layer(middleware_stack);

    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or("3000".to_string())
        .parse::<i16>()
        .expect("SERVER_PORT environment variable should be parsed correctly")
        as u16;

    let addr = SocketAddr::from(([127, 0, 0, 1], server_port));
    (app, addr)
}

fn routes() -> Router {
    Router::new()
        .route("/authors/create", post(handlers::authors::create))
        .route("/authors/update", patch(handlers::authors::update))
        .route("/authors/delete/:id", delete(handlers::authors::delete))
        .route("/authors/get/:name", get(handlers::authors::get))
        .route("/authors/login", post(handlers::authors::login))
        .route("/links/create", post(handlers::links::create))
        .route("/links/update", patch(handlers::links::update))
        .route("/links/delete/:id", delete(handlers::links::delete))
        .route("/links/get/:author_name", get(handlers::links::get))
        .route("/articles/create", post(handlers::articles::create))
        // TODO add search and update articles route
        .route("/articles/delete/:id", delete(handlers::articles::delete))
        .route("/articles/get/:permalink", get(handlers::articles::get))
        .route("/articles/tag/:tag_id", get(handlers::articles::tag))
        .route(
            "/articles/author/:author_id",
            get(handlers::articles::author),
        )
        .route("/tags/create/:article_id", post(handlers::tags::create))
        .route(
            "/tags/delete/:article_id/:tag_id",
            delete(handlers::tags::delete),
        )
        .route("/tags/get/:article_permalink", get(handlers::tags::get))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::authors::{Author, LoginAuthor, NewAuthor, UpdateAuthor};
    use crate::entities::links::{Link, NewLink};
    use axum::body::Body;
    use axum::http;
    use axum::http::{Request, StatusCode};
    use serde_json::{json, Value};
    use std::env;
    use tower::ServiceExt;

    #[tokio::test]
    async fn integration_tests() {
        env::set_var("RUST_LOG", "none");
        let new_author: NewAuthor = NewAuthor {
            name: "tomoko".to_string(),
            email: "tomokoaran@heiwa.jp".to_string(),
            display_name: "Tomoko Aran".to_string(),
            password: "midnight".to_string(),
        };
        let login_author_failed: LoginAuthor = LoginAuthor {
            name: "minako".to_string(),
            password: "midnight".to_string(),
        };
        let login_author: LoginAuthor = LoginAuthor {
            name: "minako".to_string(),
            password: "pretenders".to_string(),
        };
        let new_link: NewLink = NewLink {
            url: "https://pedro.tokyo".to_string(),
            title: "Ayuni D;".to_string(),
            author_id: 1,
        };

        let app = init_server(routes()).0;

        // Create author
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authors/create")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(new_author)).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let author: Author = serde_json::from_str(&body.to_string()).unwrap();

        // Create author error name exist
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authors/create")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(new_author)).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // Update author name and password
        let update_author: UpdateAuthor = UpdateAuthor {
            id: author.id,
            name: Option::from("minako".to_string()),
            email: None,
            display_name: None,
            password: Option::from("pretenders".to_string()),
        };
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::PATCH)
                    .uri("/authors/update")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(update_author)).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Get author
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/authors/get/{}", update_author.name.unwrap()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Login author
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authors/login")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(login_author)).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Login author failed
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authors/login")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(login_author_failed)).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Delete author
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/authors/delete/{}", author.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Create link
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/links/create")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(new_link)).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let link: Link = serde_json::from_str(&body.to_string()).unwrap();

        // Get admin user links
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/links/get/{}", "admin"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Update link
        let update_link: Link = Link {
            id: link.id,
            url: "htps://numbergirl.com".to_string(),
            title: "Number Girl".to_string(),
            author_id: link.author_id,
        };
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::PATCH)
                    .uri("/links/update")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(update_link)).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Delete link
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/links/delete/{}", link.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
