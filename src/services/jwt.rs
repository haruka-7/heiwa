use crate::CONFIG;
use chrono::{Duration, Utc};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(author_id: i32) -> Self {
        let iat = Utc::now();
        // token expire set to 6 months
        let exp = iat + Duration::hours(4383);
        Self {
            sub: author_id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(author_id: i32) -> Result<String, ()> {
    let token_encoded = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(author_id),
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    );
    match token_encoded {
        Ok(token) => Ok(token),
        Err(_) => Err(()),
    }
}

// TODO update token/session expiration
pub fn verify(token: &str, author_id: i32) -> Result<(), String> {
    let token_decoded = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data: TokenData<Claims>| data.claims);
    match token_decoded {
        Ok(claims) => {
            if claims.sub == author_id {
                Ok(())
            } else {
                Err("Invalid JWT claims".to_string())
            }
        },
        Err(_) => Err("Invalid JWT token".to_string()),
    }
}
