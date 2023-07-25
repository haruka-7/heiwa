use crate::templates::{BackArticleNewTemplate, BackArticlesListTemplate};
use axum::response::Redirect;

pub async fn list() -> BackArticlesListTemplate {
    BackArticlesListTemplate {
        alert: "list".to_string(),
    }
}

pub async fn new() -> BackArticleNewTemplate {
    BackArticleNewTemplate {
        alert: "new".to_string(),
    }
}

pub async fn new_action() -> Redirect {
    Redirect::to("/dashboard/articles")
}
