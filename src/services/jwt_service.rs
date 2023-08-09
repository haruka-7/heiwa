use crate::CONFIG;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub iss: i32,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(author_id: i32, author_role: String) -> Self {
        let iat = Utc::now();
        // token expire set to 6 months
        let exp = iat + Duration::hours(4383);
        Self {
            iss: author_id,
            role: author_role,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(author_id: i32, author_role: String) -> Result<String, ()> {
    let token_encoded = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(author_id, author_role),
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    );
    match token_encoded {
        Ok(token) => Ok(token),
        Err(_) => Err(()),
    }
}

// TODO update token/session expiration
pub fn verify(token: &str, author_id: String) -> Result<Claims, Error> {
    let mut validation: Validation = Validation::default();
    validation.set_issuer(&[author_id]);
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &validation,
    )
    .map(|data: TokenData<Claims>| data.claims)
}
