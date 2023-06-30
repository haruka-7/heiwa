use crate::schema::*;
use crate::services::authors::validate_unique_name;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Identifiable, Selectable, PartialEq, Serialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub display_name: String,
    pub biography: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Validate, Insertable, Deserialize)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    #[validate(custom = "validate_unique_name")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Queryable, Selectable, Deserialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginAuthorPassword {
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginAuthor {
    pub name: String,
    pub password: String,
}
