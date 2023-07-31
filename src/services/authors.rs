use crate::entities::authors::NewAuthor;
use crate::handlers::api::authors::{AuthAuthor, FormLoginAuthor};
use crate::services::jwt::verify_token;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum_sessions::extractors::WritableSession;
use hyper::StatusCode;

pub async fn auth_api_call(form_login_author: FormLoginAuthor) -> Result<AuthAuthor, ()> {
    let client = reqwest::Client::new();
    let request = client
        .post("http://localhost:3000/api/authors/login")
        .json(&form_login_author);
    let response = request.send().await.unwrap();
    tracing::debug!(
        "\nREQUEST POST http://localhost:3000/api/authors/login \n{:?}\nRESPONSE\n{:?}",
        &form_login_author,
        &response
    );
    let auth = response
        .json::<AuthAuthor>()
        .await
        .unwrap_or(AuthAuthor::default());
    if !auth.token.is_empty() {
        Ok(auth)
    } else {
        Err(())
    }
}

pub async fn create_api_call(author: NewAuthor) -> Result<(), ()> {
    let client = reqwest::Client::new();
    let request = client
        .post("http://localhost:3000/api/authors/create")
        .json(&author);
    let response = request.send().await.unwrap();
    tracing::debug!(
        "\nREQUEST POST http://localhost:3000/api/authors/create \n{:?}\nRESPONSE\n{:?}",
        &author,
        &response
    );
    if response.status() == StatusCode::CREATED {
        Ok(())
    } else {
        Err(())
    }
}

pub fn is_author_logged(session: &WritableSession) -> Result<(), ()> {
    if session.get::<String>("token").is_some() {
        match verify_token(session.get::<String>("token").unwrap()) {
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
