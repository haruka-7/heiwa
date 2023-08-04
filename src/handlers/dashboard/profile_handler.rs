use crate::entities::authors_entity::UpdateAuthor;
use crate::services::authors_service::{find_author_by_id, is_author_logged, update_author};
use crate::services::session_service::{session_insert_alert, session_remove_alert};
use crate::templates::dashboard_templates::DashboardProfileTemplate;
use crate::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use axum_sessions::extractors::WritableSession;
use std::sync::Arc;

/// TODO to move to a lang toml file
const PROFILE_UPDATE: &str = "Profil mis à jour avec succès";
const PROFILE_UPDATE_ALERT: &str = "Erreur lors de la mise à jour du profil";

pub async fn edit(State(state): State<Arc<AppState>>, mut session: WritableSession) -> Response {
    match is_author_logged(&session) {
        Ok(_) => {
            let alert_message: String = session.get::<String>("alert").unwrap_or("".to_string());
            session_remove_alert(&mut session);
            match find_author_by_id(&state, session.get::<i32>("author_id").unwrap()) {
                Ok(author) => DashboardProfileTemplate {
                    alert: alert_message,
                    author_id: author.id,
                    display_name: author.display_name,
                    biography: author.biography.unwrap_or("".to_string()),
                }
                .into_response(),
                Err(_) => Redirect::to("/error-page").into_response(),
            }
        }
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn edit_action(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Form(author): Form<UpdateAuthor>,
) -> Response {
    match is_author_logged(&session) {
        Ok(_) => match update_author(&state, author) {
            Ok(_) => {
                session_insert_alert(&mut session, PROFILE_UPDATE);
                Redirect::to("/dashboard/profile").into_response()
            }
            Err(_) => {
                session_insert_alert(&mut session, PROFILE_UPDATE_ALERT);
                Redirect::to("/dashboard/profile").into_response()
            }
        },
        Err(_) => Redirect::to("/login").into_response(),
    }
}
