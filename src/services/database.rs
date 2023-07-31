use crate::CONFIG;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection() -> PgConnection {
    let database_url = &CONFIG.database_url;
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = &CONFIG.database_url;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}