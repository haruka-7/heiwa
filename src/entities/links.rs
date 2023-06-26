use crate::entities::authors::Author;
use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq, Serialize)]
#[diesel(belongs_to(Author))]
#[diesel(table_name = links)]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub author_id: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = links)]
pub struct NewLink {
    pub url: String,
    pub title: String,
    pub author_id: i32,
}
