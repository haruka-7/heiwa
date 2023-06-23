use crate::schema::*;
use crate::services;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub biography: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Validate, Insertable, Deserialize)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    #[validate(custom = "services::validate_unique_name")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub display_name: String,
    pub password: String,
}
