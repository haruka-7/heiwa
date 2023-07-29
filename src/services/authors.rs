use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use crate::entities::authors::{AuthorAccessToken, LoginAuthor};

pub async fn auth(login_author: LoginAuthor) -> Result<String, ()> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/api/authors/login")
        .json(&login_author)
        .send()
        .await
        .unwrap()
        .json::<AuthorAccessToken>()
        .await
        .unwrap();
    if !response.access_token.is_empty() {
        Ok(response.access_token)
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
        assert!(verify_password(&password, password_hash.as_ref()))
    }
}