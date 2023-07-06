use crate::entities::authors::Author;
use crate::entities::links::{Link, NewLink};
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde_json::json;

pub async fn get(Path(author_name): Path<String>) -> Response {
    let author: Vec<Author> = Author::find_by_name(author_name);
    if author.is_empty() {
        StatusCode::NOT_FOUND.into_response()
    } else {
        let links: Vec<Link> = Link::find_by_author(author.first().unwrap());
        Json(json!(links)).into_response()
    }
}

pub async fn create(Json(payload): Json<NewLink>) -> Response {
    let link_result: QueryResult<Link> = Link::create(payload);
    match link_result {
        Ok(link) => (StatusCode::CREATED, Json(json!(link))).into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let nb_deleted: usize = Link::delete(id);
    if nb_deleted == 0 {
        tracing::error!("Can not delete link with id {}", id);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
