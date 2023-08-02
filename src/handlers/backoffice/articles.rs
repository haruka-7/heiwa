use axum::Form;
use crate::services::authors::is_author_logged;
use crate::services::session::{session_insert_alert, session_remove_alert};
use crate::templates::{BackArticleNewTemplate, BackArticlesListTemplate};
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::extractors::WritableSession;
use crate::entities::articles::{Article, NewArticle};
use crate::services::articles::{create_article_api_call, list_articles_api_call};

/// TODO to move to a lang toml file
const NEW_ARTICLE_ALERT: &str = "Erreur lors de la création de l'article";

pub async fn list(session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            let articles: Vec<Article> = list_articles_api_call(session.get::<i32>("author_id").unwrap_or(0)).await.unwrap_or(vec![]);
            BackArticlesListTemplate {
                articles,
            }
                .into_response()
        },
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn new(mut session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            let alert_message: String = session.get::<String>("alert").unwrap_or("".to_string());
            session_remove_alert(&mut session);
            BackArticleNewTemplate {
                author_id: session.get::<i32>("author_id").unwrap_or(0),
                alert: alert_message,
            }
            .into_response()
        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn new_action(mut session: WritableSession, Form(new_article): Form<NewArticle>) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            match create_article_api_call(&mut session, new_article).await {
                Ok(_) => Redirect::to("/dashboard/articles").into_response(),
                Err(_) => {
                    session_insert_alert(&mut session, NEW_ARTICLE_ALERT);
                    Redirect::to("/dashboard/article").into_response()
                },
            }

        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}
