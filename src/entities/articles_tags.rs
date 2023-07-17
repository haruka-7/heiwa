use crate::entities::articles::Article;
use crate::entities::tags::Tag;
use crate::schema::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use serde::Serialize;
use crate::services::database::establish_connection;

#[derive(
    Debug, Queryable, Insertable, Selectable, Identifiable, Associations, PartialEq, Serialize,
)]
#[diesel(belongs_to(Article))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = articles_tags)]
#[diesel(primary_key(article_id, tag_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArticleTag {
    pub article_id: i32,
    pub tag_id: i32,
}

impl ArticleTag {
    pub fn create(new_article_tag: ArticleTag) -> QueryResult<ArticleTag> {
        insert_into(articles_tags::table)
            .values(&new_article_tag)
            .returning(ArticleTag::as_returning())
            .get_result(&mut establish_connection())
    }

    pub fn delete(article_id: i32, tag_id: i32) -> QueryResult<usize> {
        delete(ArticleTag::table().find((article_id, tag_id)))
            .execute(&mut establish_connection())
    }
}
