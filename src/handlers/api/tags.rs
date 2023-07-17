use crate::entities::articles::Article;
use crate::entities::articles_tags::ArticleTag;
use crate::entities::tags::{NewTag, Tag};
use axum::extract::Path;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use diesel::QueryResult;
use serde_json::json;
use crate::handlers::api::errors::handle_error;

pub async fn get(Path(article_permalink): Path<String>) -> Response {
    let article_result: QueryResult<Vec<Article>> = Article::find_by_permalink(article_permalink);
    match article_result {
        Ok(article) => {
            if article.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                let tags_result: QueryResult<Vec<Tag>> = Tag::find_tags_by_article(article.first().unwrap().id);
                match tags_result {
                    Ok(tags) => {
                        Json(json!(tags)).into_response()
                    }
                    Err(e) => {
                        handle_error(e)
                    }
                }
            }
        }
        Err(e) => {
            handle_error(e)
        }
    }
}

pub async fn create(Path(article_id): Path<i32>, Json(payload): Json<NewTag>) -> Response {
    let tag_result: QueryResult<Tag> = Tag::create(payload);
    match tag_result {
        Ok(tag) => {
            let article_tag: ArticleTag = ArticleTag {
                article_id,
                tag_id: tag.id,
            };
            let article_tag_result: QueryResult<ArticleTag> = ArticleTag::create(article_tag);
            match article_tag_result {
                Ok(_) => {
                    StatusCode::CREATED.into_response()
                }
                Err(e) => {
                    tracing::error!("Can not insert article-tag relation with error {}", e);
                    // Rollback Tag insertion if ArticleTag can not be inserted
                    let delete_result: QueryResult<usize> = Tag::delete(tag.id);
                    match delete_result {
                        Ok(_) => {}
                        Err(e) => {
                            return handle_error(e);
                        }
                    }
                    handle_error(e)
                }
            }
        }
        Err(e) => {
            handle_error(e)
        }
    }
}

pub async fn delete(Path(article_id): Path<i32>, Path(tag_id): Path<i32>) -> Response {
    let delete_result: QueryResult<usize> = ArticleTag::delete(article_id, tag_id);
    match delete_result {
        Ok(nb_relations_deleted) => {
            if nb_relations_deleted == 0 {
                tracing::error!(
                    "Can not delete article-tag relation with article_id {} and tag_id {}",
                    article_id,
                    tag_id
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                // Remove Tag if any article used it
                let articles_result: QueryResult<Vec<Article>> = Article::find_by_tag(tag_id);
                match articles_result {
                    Ok(articles) => {
                        if articles.is_empty() {
                            let delete_result: QueryResult<usize> = Tag::delete(tag_id);
                            match delete_result {
                                Ok(_) => {
                                    StatusCode::OK.into_response()
                                }
                                Err(e) => {
                                    handle_error(e)
                                }
                            }
                        } else {
                            StatusCode::OK.into_response()
                        }
                    }
                    Err(e) => {
                        handle_error(e)
                    }
                }
            }
        }
        Err(e) => {
            handle_error(e)
        }
    }
}
