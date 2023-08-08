use crate::entities::authors_entity::{Author, NewAuthor, UpdateAuthor};
use crate::entities::roles_entity::Roles;
use crate::handlers::api::authors_api_handler::{AuthAuthor, FormLoginAuthor};
use crate::services::jwt_service;
use crate::services::jwt_service::verify;
use crate::AppState;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum_sessions::extractors::WritableSession;
use diesel::QueryResult;
use std::sync::Arc;
use validator::{Validate, ValidationError};

pub fn find_author_by_id(state: &Arc<AppState>, id: i32) -> Result<Author, ()> {
    let author_result: QueryResult<Vec<Author>> =
        Author::find_by_id(state.db_connection.get().unwrap(), id);
    match_author_result(author_result)
}

pub fn find_author_by_name(state: &Arc<AppState>, name: String) -> Result<Author, ()> {
    let author_result: QueryResult<Vec<Author>> =
        Author::find_by_name(state.db_connection.get().unwrap(), name);
    match_author_result(author_result)
}

fn find_by_name_or_email(state: &Arc<AppState>, name: String, email: String) -> Result<Author, ()> {
    let author_result =
        Author::find_by_name_or_email(state.db_connection.get().unwrap(), name, email);
    match_author_result(author_result)
}

pub fn auth_author(
    state: &Arc<AppState>,
    form_login_author: FormLoginAuthor,
) -> Result<AuthAuthor, ()> {
    match find_author_by_name(&state, form_login_author.name) {
        Ok(author) => match verify_password(&form_login_author.password, &author.password) {
            Ok(_) => {
                let jwt_token = jwt_service::sign(
                    author.id,
                    author.role.clone().unwrap_or(Roles::Author.to_string()),
                )
                .unwrap();
                Ok(AuthAuthor::new(
                    author.id,
                    jwt_token,
                    author.role.clone().unwrap_or(Roles::Author.to_string()),
                ))
            }
            Err(_) => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub fn create_author(state: &Arc<AppState>, author: NewAuthor) -> Result<(), Option<String>> {
    match author.validate() {
        Ok(_) => match validate_unique_name_and_email(&state, &author.name, &author.email) {
            Ok(_) => {
                let author_result: QueryResult<Author> =
                    Author::create(state.db_connection.get().unwrap(), author);
                match author_result {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        tracing::error!("{}", e);
                        Err(Some("TECHNICAL_ERROR".to_string()))
                    }
                }
            }
            Err(e) => Err(Some(e.code.to_string())),
        },
        Err(_) => Err(None),
    }
}

pub fn update_author(state: &Arc<AppState>, author: UpdateAuthor) -> Result<(), Option<String>> {
    match author.validate() {
        Ok(_) => match validate_unique_name_and_email(
            &state,
            &author.name.clone().unwrap_or("".to_string()),
            &author.email.clone().unwrap_or("".to_string()),
        ) {
            Ok(_) => {
                let author_result: QueryResult<usize> =
                    Author::update(state.db_connection.get().unwrap(), author);
                match author_result {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        tracing::error!("{}", e);
                        Err(Some("TECHNICAL_ERROR".to_string()))
                    }
                }
            }
            Err(e) => Err(Some(e.code.to_string())),
        },
        Err(_) => Err(None),
    }
}

pub fn is_author_logged(session: &WritableSession) -> Result<(), ()> {
    let (token, id) = (
        session.get::<String>("token").unwrap_or("".to_string()),
        session.get::<i32>("author_id").unwrap_or(0),
    );
    if !token.is_empty() {
        match verify(token.as_str(), id) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    } else {
        Err(())
    }
}

pub fn hash_password(password: &String) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2: Argon2 = Argon2::default();
    argon2
        .hash_password(password.as_ref(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(
    password: &String,
    password_hash: &str,
) -> argon2::password_hash::Result<()> {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Argon2::default().verify_password(password.as_ref(), &parsed_hash)
}

pub fn validate_unique_name_and_email(
    state: &Arc<AppState>,
    name: &str,
    email: &str,
) -> Result<(), ValidationError> {
    match find_by_name_or_email(&state, name.to_string(), email.to_string()) {
        Ok(_) => Err(ValidationError::new("NAME_OR_EMAIL_EXIST")),
        Err(_) => Ok(()),
    }
}

fn match_author_result(author_result: QueryResult<Vec<Author>>) -> Result<Author, ()> {
    match author_result {
        Ok(mut authors) => {
            if !authors.is_empty() {
                Ok(authors.pop().unwrap())
            } else {
                Err(())
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password: String = "thee michel gun elephant".to_string();
        let password_hash: String = hash_password(&password);
        assert!(verify_password(&password, password_hash.as_ref()).is_ok())
    }
}
