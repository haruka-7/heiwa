use crate::entities::authors::Author;
use crate::schema::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use heiwa_common::utils::establish_connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq, Serialize, Deserialize)]
#[diesel(belongs_to(Author))]
#[diesel(table_name = links)]
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
    pub fn find_by_author(author: &Author) -> Vec<Link> {
        Link::belonging_to(&author)
            .select(Link::as_select())
            .load(&mut establish_connection())
            .expect("Should load handlers")
    }

    pub fn create(new_link: NewLink) -> QueryResult<Link> {
        insert_into(links::table)
            .values(&new_link)
            .returning(Link::as_returning())
            .get_result(&mut establish_connection())
    }

    pub fn delete(id: i32) -> usize {
        delete(Link::table().find(id))
            .execute(&mut establish_connection())
            .expect("Should delete link")
    }
}
