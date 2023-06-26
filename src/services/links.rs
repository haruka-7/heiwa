use crate::entities::authors::Author;
use crate::entities::links::{Link, NewLink};
use crate::schema::links;
use diesel::{
    insert_into, pg::PgConnection, BelongingToDsl, QueryDsl, QueryResult, RunQueryDsl,
    SelectableHelper,
};

pub fn get_links_by_author(connection: &mut PgConnection, author: &Author) -> Vec<Link> {
    Link::belonging_to(&author)
        .select(Link::as_select())
        .load(connection)
        .expect("Should load handlers")
}

pub fn create_link(connection: &mut PgConnection, new_link: NewLink) -> QueryResult<Link> {
    insert_into(links::table)
        .values(&new_link)
        .returning(Link::as_returning())
        .get_result(connection)
}
