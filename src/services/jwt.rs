use crate::entities::authors::Author;
use crate::CONFIG;
use chrono::{Duration, Utc};
use diesel::QueryResult;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(name: String) -> Self {
        let iat = Utc::now();
        // token expire set to 6 months
        let exp = iat + Duration::hours(4383);
        Self {
            sub: name,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(name: String) -> Result<String, ()> {
    let token_encoded = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(name),
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    );
    match token_encoded {
        Ok(token) => Ok(token),
        Err(_) => Err(()),
    }
}

pub fn verify(token: &str) -> Result<Claims, String> {
    let token_decoded = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims);
    match token_decoded {
        Ok(claims) => Ok(claims),
        Err(_) => Err("Invalid JWT token".to_string()),
    }
}

pub fn verify_token(token: String) -> QueryResult<Vec<Author>> {
    let claims = verify(&token).unwrap();
    Author::find_by_name(claims.sub)
}
