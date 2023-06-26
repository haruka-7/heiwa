use crate::entities::authors::Author;
use crate::entities::links::{Link, NewLink};
use crate::services::authors::get_authors_by_name;
use crate::services::links::{create_link, delete_link, get_links_by_author};
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use heiwa_common::utils::establish_connection;
use serde_json::json;

pub async fn get(Path(author_name): Path<String>) -> Response {
    let author: Vec<Author> = get_authors_by_name(&mut establish_connection(), author_name);
    if author.is_empty() {
        StatusCode::NOT_FOUND.into_response()
    } else {
        let links: Vec<Link> =
            get_links_by_author(&mut establish_connection(), author.first().unwrap());
        Json(json!(links)).into_response()
    }
}

pub async fn create(Json(payload): Json<NewLink>) -> Response {
    let link: QueryResult<Link> = create_link(&mut establish_connection(), payload);
    match link {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    let nb_deleted: usize = delete_link(&mut establish_connection(), id);
    if nb_deleted == 0 {
        tracing::error!("Can not delete link with id {}", id);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
