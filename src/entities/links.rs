use crate::entities::authors::Author;
use crate::schema::*;
use crate::services::database::establish_connection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    Debug,
    Queryable,
    Selectable,
    Identifiable,
    AsChangeset,
    Associations,
    PartialEq,
    Serialize,
    Deserialize,
    Validate,
)]
#[diesel(belongs_to(Author))]
pub struct Link {
    pub id: i32,
    #[validate(url)]
    pub url: String,
    pub title: String,
    pub author_id: i32,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = links)]
pub struct NewLink {
    #[validate(url)]
    pub url: String,
    pub title: String,
    pub author_id: i32,
}

impl Link {
    pub fn find_by_author(author: &Author) -> QueryResult<Vec<Link>> {
        Link::belonging_to(&author)
            .select(Link::as_select())
            .load(&mut establish_connection())
    }

    pub fn create(new_link: NewLink) -> QueryResult<Link> {
        insert_into(links::table)
            .values(&new_link)
            .returning(Link::as_returning())
            .get_result(&mut establish_connection())
    }

    pub fn update(update_link: Link) -> QueryResult<usize> {
        update(&update_link)
            .set(&update_link)
            .execute(&mut establish_connection())
    }

    pub fn delete(id: i32) -> QueryResult<usize> {
        delete(Link::table().find(id)).execute(&mut establish_connection())
    }
}
