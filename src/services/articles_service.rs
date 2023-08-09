use crate::entities::articles_entity::{Article, NewArticle};
use crate::AppState;
use diesel::QueryResult;
use std::sync::Arc;
use validator::ValidationError;

//TODO add a limit or pagination
pub fn find_articles(state: &Arc<AppState>) -> Result<Vec<Article>, Option<String>> {
    let articles_result = Article::find(state.db_connection.get().unwrap());
    match articles_result {
        Ok(articles) => {
            if !articles.is_empty() {
                Ok(articles)
            } else {
                Err(Some("NO_ARTICLE_FOUND".to_string()))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(None)
        }
    }
}

pub fn find_articles_by_author(
    state: &Arc<AppState>,
    author_id: i32,
) -> Result<Vec<Article>, Option<String>> {
    let articles_result = Article::find_by_author(state.db_connection.get().unwrap(), author_id);
    match articles_result {
        Ok(articles) => {
            if !articles.is_empty() {
                Ok(articles)
            } else {
                Err(Some("NO_ARTICLE_FOUND".to_string()))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(None)
        }
    }
}

pub fn find_article_by_permalink(
    state: &Arc<AppState>,
    article_permalink: String,
) -> Result<Article, Option<String>> {
    let articles_result =
        Article::find_by_permalink(state.db_connection.get().unwrap(), article_permalink);
    match articles_result {
        Ok(mut articles) => {
            if !articles.is_empty() {
                Ok(articles.pop().unwrap())
            } else {
                Err(Some("NO_ARTICLE_FOUND".to_string()))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(None)
        }
    }
}

pub fn create_article(state: &Arc<AppState>, article: NewArticle) -> Result<(), Option<String>> {
    match validate_unique_permalink(state, &article.permalink) {
        Ok(_) => {
            let article_result: QueryResult<Article> =
                Article::create(state.db_connection.get().unwrap(), article);
            match article_result {
                Ok(_) => Ok(()),
                Err(e) => {
                    tracing::error!("{}", e);
                    Err(Some("TECHNICAL_ERROR".to_string()))
                }
            }
        }
        Err(e) => Err(Some(e.code.to_string())),
    }
}

pub fn update_article(state: &Arc<AppState>, article: NewArticle) -> Result<(), Option<String>> {
    let article_result: QueryResult<usize> =
        Article::update(state.db_connection.get().unwrap(), article);
    match article_result {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("{}", e);
            Err(Some("TECHNICAL_ERROR".to_string()))
        }
    }
}

fn validate_unique_permalink(
    state: &Arc<AppState>,
    permalink: &str,
) -> Result<(), ValidationError> {
    let article_result: QueryResult<Vec<Article>> =
        Article::find_by_permalink(state.db_connection.get().unwrap(), permalink.to_string());
    match article_result {
        Ok(articles) => {
            if articles.is_empty() {
                Ok(())
            } else {
                Err(ValidationError::new("PERMALINK_EXIST"))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(ValidationError::new("VALIDATE_PERMALINK_ERROR"))
        }
    }
}
