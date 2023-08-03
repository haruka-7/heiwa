use crate::entities::authors::{Author, NewAuthor};
use crate::entities::roles::Roles;
use crate::handlers::api::authors::{AuthAuthor, FormLoginAuthor};
use crate::services::jwt;
use crate::services::jwt::verify;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum_sessions::extractors::WritableSession;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, QueryResult};
use validator::{Validate, ValidationError};

pub fn find_author_by_name(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
) -> Result<Vec<Author>, ()> {
    let author_result = Author::find_by_name(connection, name);
    match author_result {
        Ok(authors) => {
            if !authors.is_empty() {
                Ok(authors)
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

fn find_by_name_or_email(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    email: String,
) -> Result<Vec<Author>, ()> {
    let author_result = Author::find_by_name_or_email(connection, name, email);
    match author_result {
        Ok(authors) => {
            if !authors.is_empty() {
                Ok(authors)
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

pub fn auth_author(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    form_login_author: FormLoginAuthor,
) -> Result<AuthAuthor, ()> {
    match find_author_by_name(connection, form_login_author.name) {
        Ok(authors) => {
            let author: &Author = authors.first().unwrap();
            match verify_password(&form_login_author.password, &author.password) {
                Ok(_) => {
                    let jwt_token = jwt::sign(author.id).unwrap();
                    Ok(AuthAuthor::new(
                        author.id,
                        jwt_token,
                        author.role.clone().unwrap_or(Roles::Author.to_string()),
                    ))
                }
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

pub fn create_author(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    author: NewAuthor,
) -> Result<(), Option<String>> {
    match author.validate() {
        Ok(_) => match validate_unique_name_and_email(connection, &author.name, &author.email) {
            Ok(_) => {
                let author_result: QueryResult<Author> = Author::create(author);
                match author_result {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        tracing::error!("{}", e);
                        Err(None)
                    }
                }
            }
            Err(e) => {
                tracing::error!("{}", e);
                Err(Some(e.code.to_string()))
            }
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
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    name: &str,
    email: &str,
) -> Result<(), ValidationError> {
    match find_by_name_or_email(connection, name.to_string(), email.to_string()) {
        Ok(_) => Err(ValidationError::new("NAME_OR_EMAIL_EXIST")),
        Err(_) => Ok(()),
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
