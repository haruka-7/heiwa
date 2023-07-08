use crate::entities::authors::Author;
use crate::schema::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use heiwa_common::utils::establish_connection;
use serde::{Deserialize, Serialize};

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
)]
#[diesel(belongs_to(Author))]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub author_id: i32,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = links)]
pub struct NewLink {
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
        update(links::table)
            .set(&update_link)
            .execute(&mut establish_connection())
    }

    pub fn delete(id: i32) -> QueryResult<usize> {
        delete(Link::table().find(id)).execute(&mut establish_connection())
    }
}
