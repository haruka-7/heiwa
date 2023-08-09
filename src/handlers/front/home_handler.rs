use crate::services::articles_service::find_articles;
use crate::services::authors_service::find_author_by_id;
use crate::templates::site_templates::{ArticleData, HomeTemplate};
use crate::AppState;
use axum::extract::State;
use std::sync::Arc;

pub async fn show(State(state): State<Arc<AppState>>) -> HomeTemplate {
    match find_articles(&state) {
        Ok(articles) => {
            let mut articles_data: Vec<ArticleData> = Vec::new();
            for article in articles {
                let author_name: String =
                    find_author_by_id(&state, article.author_id).unwrap().name;
                articles_data.push(ArticleData::new(article, author_name));
            }
            HomeTemplate {
                title: "Welcome".to_string(),
                alert: "".to_string(),
                articles: articles_data,
            }
        }
        Err(error) => HomeTemplate {
            title: "Welcome".to_string(),
            alert: error.unwrap_or("".to_string()),
            articles: vec![],
        },
    }
}
