use crate::entities::articles_tags_entity::ArticleTag;
use crate::entities::tags_entity::Tag;
use crate::schema::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{delete, insert_into, update};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use sanitizer::prelude::Sanitize;
use sanitizer::StringSanitizer;

#[derive(Debug, Queryable, Identifiable, Selectable, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub permalink: String,
    pub title: String,
    pub creation_date: SystemTime,
    pub publication_date: Option<SystemTime>,
    pub update_date: Option<SystemTime>,
    pub content: Option<String>,
    pub published: bool,
    pub meta_description: Option<String>,
    pub author_id: i32,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize, Sanitize)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    #[sanitize(trim, kebab_case)]
    pub permalink: String,
    #[sanitize(trim)]
    pub title: String,
    #[sanitize(trim)]
    pub content: Option<String>,
    #[sanitize(trim, clamp(100))]
    pub meta_description: Option<String>,
    pub author_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct FormNewArticle {
    pub title: String,
    pub content: String,
    pub author_id: i32,
}

impl Article {
    pub fn find(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<Vec<Article>> {
        Article::table()
            .select(Article::as_select())
            .load(&mut connection)
    }

    pub fn find_by_permalink(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        permalink_param: String,
    ) -> QueryResult<Vec<Article>> {
        Article::table()
            .filter(articles::permalink.eq(permalink_param))
            .limit(1)
            .select(Article::as_select())
            .load(&mut connection)
    }

    pub fn find_by_author(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        author_param: i32,
    ) -> QueryResult<Vec<Article>> {
        Article::table()
            .filter(articles::author_id.eq(author_param))
            .select(Article::as_select())
            .load(&mut connection)
    }

    pub fn find_by_tag(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        tag: &Tag,
    ) -> QueryResult<Vec<Article>> {
        ArticleTag::belonging_to(&tag)
            .inner_join(articles::table)
            .select(Article::as_select())
            .load(&mut connection)
    }

    pub fn create(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        new_article: NewArticle,
    ) -> QueryResult<Article> {
        insert_into(articles::table)
            .values(&new_article)
            .returning(Article::as_returning())
            .get_result(&mut connection)
    }

    pub fn update(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        article: NewArticle,
    ) -> QueryResult<usize> {
        update(articles::table)
            .set(article)
            .execute(&mut connection)
    }

    pub fn delete(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        id: i32,
    ) -> QueryResult<usize> {
        delete(Article::table().find(id)).execute(&mut connection)
    }
}

impl NewArticle {
    pub fn from_form(form_article: FormNewArticle) -> Self {
        Self {
            title: form_article.title.clone(),
            content: Some(form_article.content.clone()),
            permalink: form_article.title,
            meta_description: Some(form_article.content),
            author_id: form_article.author_id,
        }
    }
}
