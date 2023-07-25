use crate::CONFIG;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = &CONFIG.database_url;
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
