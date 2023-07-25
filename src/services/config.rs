use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub rust_log: String,
    pub server_port: u16,
    pub server_timeout: u64,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            rust_log: env::var("RUST_LOG").unwrap_or("ERROR".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or("3000".to_string())
                .parse()
                .unwrap(),
            server_timeout: env::var("SERVER_TIMEOUT")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap(),
            database_url: "".to_string(),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expires_in: env::var("JWT_EXPIRED_IN").unwrap_or("60m".to_string()),
            jwt_maxage: env::var("JWT_MAXAGE")
                .unwrap_or("60".to_string())
                .parse::<i32>()
                .unwrap(),
        }
    }
}
