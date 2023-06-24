use crate::schema::*;
use crate::services;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub author_id: i32,
}

#[derive(Debug, Validate, Insertable, Deserialize)]
#[diesel(table_name = links)]
pub struct NewLink {
    #[validate(custom = "services::links::validate_unique_url")]
    pub url: String,
    pub title: String,
    pub author_id: i32,
}