use crate::entities::articles_entity::{FormNewArticle};
use crate::services::articles_service::{
    create_article, find_article_by_permalink, find_articles_by_author, update_article,
};
use crate::services::authors_service::{find_author_by_id, is_author_logged};
use crate::services::session_service::{session_insert_alert, session_remove_alert};
use crate::templates::dashboard_templates::{
    DashboardArticleNewTemplate, DashboardArticlesListTemplate,
};
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use std::sync::Arc;
use crate::templates::site_templates::ArticleData;

/// TODO to move to a lang toml file
const NEW_ARTICLE_ALERT: &str = "Erreur lors de la création de l'article";
const ARTICLE_UPDATE_ALERT: &str = "Erreur lors de la mise à jour de l'article";
const ARTICLE_CREATED: &str = "Article créé avec succès";
const ARTICLE_UPDATED: &str = "Article mise à jour avec succès";

pub async fn list(State(state): State<Arc<AppState>>, mut session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            match find_articles_by_author(&state, session.get::<i32>("author_id").unwrap_or(0)) {
                Ok(articles) => {
                    let alert_message: String =
                        session.get::<String>("alert").unwrap_or("".to_string());
                    session_remove_alert(&mut session);
                    let mut articles_data: Vec<ArticleData> = Vec::new();
                    for article in articles {
                        let author_name: String =
                            find_author_by_id(&state, article.author_id).unwrap().name;
                        articles_data.push(ArticleData::new(article, author_name));
                    }
                    DashboardArticlesListTemplate {
                        alert: alert_message,
                        articles: articles_data,
                    }
                    .into_response()
                }
                Err(error) => match error {
                    None => DashboardArticlesListTemplate {
                        alert: "".to_string(),
                        articles: vec![],
                    }
                    .into_response(),
                    Some(code) => DashboardArticlesListTemplate {
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
            DashboardArticleNewTemplate {
                action: "/dashboard/article".to_string(),
                author_id: session.get::<i32>("author_id").unwrap_or(0),
                title: "".to_string(),
                content: "".to_string(),
                permalink: "".to_string(),
                alert: alert_message,
                meta_description: "".to_string(),
            }
            .into_response()
        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn edit(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Path(permalink): Path<String>,
) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            let alert_message: String = session.get::<String>("alert").unwrap_or("".to_string());
            session_remove_alert(&mut session);
            match find_article_by_permalink(&state, permalink) {
                Ok(article) => DashboardArticleNewTemplate {
                    action: "/dashboard/article/update".to_string(),
                    alert: alert_message,
                    author_id: session.get::<i32>("author_id").unwrap_or(0),
                    title: article.title,
                    content: article.content.unwrap_or("".to_string()),
                    permalink: article.permalink,
                    meta_description: article.meta_description.unwrap_or("".to_string()),
                }
                .into_response(),
                Err(_) => Redirect::to("/error-page").into_response(),
            }
        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn new_action(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Form(form_article): Form<FormNewArticle>,
) -> Response {
    match is_author_logged(&session) {
        Ok(_) => match create_article(&state, form_article) {
            Ok(_) => {
                session_insert_alert(&mut session, ARTICLE_CREATED);
                Redirect::to("/dashboard/articles").into_response()
            }
            Err(error) => match error {
                None => {
                    session_insert_alert(&mut session, NEW_ARTICLE_ALERT);
                    Redirect::to("/dashboard/article").into_response()
                }
                Some(code) => {
                    session_insert_alert(&mut session, code.as_str());
                    Redirect::to("/dashboard/article").into_response()
                }
            },
        },
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn edit_action(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Form(form_article): Form<FormNewArticle>,
) -> Response {
    match is_author_logged(&session) {
        Ok(_) => match update_article(&state, form_article) {
            Ok(_) => {
                session_insert_alert(&mut session, ARTICLE_UPDATED);
                Redirect::to("/dashboard/articles").into_response()
            }
            Err(_) => {
                session_insert_alert(&mut session, ARTICLE_UPDATE_ALERT);
                Redirect::to("/dashboard/articles").into_response()
            }
        },
        Err(_) => Redirect::to("/login").into_response(),
    }
}
