use crate::entities::articles_entity::NewArticle;
use crate::services::articles_service::{create_article, find_article_by_permalink, find_articles_by_author};
use crate::services::authors_service::is_author_logged;
use crate::services::session_service::{session_insert_alert, session_remove_alert};
use crate::templates::{BackArticleNewTemplate, BackArticlesListTemplate};
use crate::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use std::sync::Arc;

/// TODO to move to a lang toml file
const NEW_ARTICLE_ALERT: &str = "Erreur lors de la création de l'article";

pub async fn list(State(state): State<Arc<AppState>>, session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            match find_articles_by_author(&state, session.get::<i32>("author_id").unwrap_or(0)) {
                Ok(articles) => BackArticlesListTemplate {
                    alert: "".to_string(),
                    articles,
                }
                .into_response(),
                Err(error) => match error {
                    None => BackArticlesListTemplate {
                        alert: "".to_string(),
                        articles: vec![],
                    }
                    .into_response(),
                    Some(code) => BackArticlesListTemplate {
                        alert: code,
                        articles: vec![],
                    }
                    .into_response(),
                },
            }
        }
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

pub async fn edit(State(state): State<Arc<AppState>>, mut session: WritableSession, permalink: String) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            let alert_message: String = session.get::<String>("alert").unwrap_or("".to_string());
            session_remove_alert(&mut session);
            match find_article_by_permalink(&state, &permalink) {
                Ok(_) => {
                    //update

                }
                Err(_) => {
                    //return alert
                }
            }
            BackArticleNewTemplate {
                author_id: session.get::<i32>("author_id").unwrap_or(0),
                alert: alert_message,
            }
                .into_response()
        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn new_action(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Form(new_article): Form<NewArticle>,
) -> Response {
    match is_author_logged(&session) {
        Ok(_) => match create_article(&state, new_article) {
            Ok(_) => Redirect::to("/dashboard/articles").into_response(),
            Err(_) => {
                session_insert_alert(&mut session, NEW_ARTICLE_ALERT);
                Redirect::to("/dashboard/article").into_response()
            }
        },
        Err(_) => Redirect::to("/login").into_response(),
    }
}
