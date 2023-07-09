use axum::routing::{delete, get, patch, post};
use axum::Router;
use heiwa_common::init_server;

mod entities;
mod handlers;
mod schema;

#[tokio::main]
async fn main() {
    let (app, addr) = init_server(routes());

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
            name: "tomoko".to_string(),
            password: "midnight".to_string(),
        };
        let login_author: LoginAuthor = LoginAuthor {
            name: "tomoko".to_string(),
            password: "pretendes".to_string(),
        };
        let new_link: NewLink = NewLink {
            url: "april.org".to_string(),
            title: "April".to_string(),
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
            name: "minako".to_string(),
            email: author.email,
            display_name: author.display_name,
            password: Option::from("pretenders".to_string()),
        };
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::PATCH)
                    .uri("/authors/update")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(update_author)).unwrap()))
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
                    .uri(format!("/authors/get/{}", update_author.name))
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

        // Delete link
        let response = app
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
