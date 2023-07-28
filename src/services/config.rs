use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub rust_log: String,
    pub server_port: u16,
    pub server_timeout: u64,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            rust_log: env::var("RUST_LOG").unwrap_or("ERROR".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or("3000".to_string())
                .parse()
                .unwrap(),
            server_timeout: env::var("SERVER_TIMEOUT")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("DATABASE_URL must be set"),
        }
    }
}
