use crate::entities::authors::NewAuthor;
use crate::handlers::api::authors::{AuthAuthor, FormLoginAuthor};
use crate::services::jwt::verify;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum_sessions::extractors::WritableSession;
use hyper::StatusCode;
use crate::services::http_client::build_http_client;

pub async fn auth_api_call(form_login_author: FormLoginAuthor) -> Result<AuthAuthor, ()> {
    let client = build_http_client();
    let request = client
        .post("http://localhost:3000/api/authors/login")
        .json(&form_login_author);
    let response = request.send().await.unwrap();
    tracing::debug!(
        "\nREQUEST POST http://localhost:3000/api/authors/login \n{:?}\nRESPONSE\n{:?}",
        &form_login_author,
        &response
    );
    let auth_author: AuthAuthor = response
        .json::<AuthAuthor>()
        .await
        .unwrap_or(AuthAuthor::default());
    if !auth_author.token.is_empty() {
        Ok(auth_author)
    } else {
        Err(())
    }
}

pub async fn create_author_api_call(author: NewAuthor) -> Result<(), ()> {
    let client = build_http_client();
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

pub async fn find_author_api_call(name: String) -> Result<(), ()> {
    let client = build_http_client();
    let request = client.get(format!("http://localhost:3000/api/authors/get/{}", &name));
    let response = request.send().await.unwrap();
    tracing::debug!(
        "\nREQUEST GET http://localhost:3000/api/authors/get/{}\nRESPONSE\n{:?}",
        &name,
        &response
    );
    if response.status() == StatusCode::OK {
        Ok(())
    } else {
        Err(())
    }
}

pub fn is_author_logged(session: &WritableSession) -> Result<(), ()> {
    let (token, id) = (session.get::<String>("token").unwrap_or("".to_string()), session.get::<i32>("author_id").unwrap_or(0));
    if !token.is_empty() {
        match verify(
            token.as_str(),
            id,
        ) {
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
