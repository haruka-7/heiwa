use crate::services::config::Config;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let config: Config = Config::new();
    let database_url = config.database_url;
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
