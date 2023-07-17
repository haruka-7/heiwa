use crate::entities::authors::Author;
use crate::entities::links::{Link, NewLink};
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde_json::json;
use crate::handlers::errors::handle_error;

pub async fn get(Path(author_name): Path<String>) -> Response {
    let author_result: QueryResult<Vec<Author>> = Author::find_by_name(author_name);
    match author_result {
        Ok(author) => {
            if author.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                let links_result: QueryResult<Vec<Link>> =
                    Link::find_by_author(author.first().unwrap());
                match links_result {
                    Ok(links) => Json(json!(links)).into_response(),
                    Err(e) => handle_error(e),
                }
            }
        }
        Err(e) => handle_error(e),
    }
}

pub async fn create(Json(payload): Json<NewLink>) -> Response {
    let link_result: QueryResult<Link> = Link::create(payload);
    match link_result {
        Ok(link) => (StatusCode::CREATED, Json(json!(link))).into_response(),
        Err(e) => handle_error(e),
    }
}

pub async fn update(Json(payload): Json<Link>) -> Response {
    let update_result: QueryResult<usize> = Link::update(payload);
    match update_result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => handle_error(e),
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let delete_result: QueryResult<usize> = Link::delete(id);
    match delete_result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => handle_error(e),
    }
}
