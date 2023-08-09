use crate::entities::articles_entity::Article;
use crate::entities::tags_entity::Tag;
use crate::schema::*;
use crate::services::database_service::connection_pool;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use serde::Serialize;

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
            .get_result(&mut connection_pool().get().unwrap())
    }

    pub fn delete(article_id: i32, tag_id: i32) -> QueryResult<usize> {
        delete(ArticleTag::table().find((article_id, tag_id)))
            .execute(&mut connection_pool().get().unwrap())
    }
}