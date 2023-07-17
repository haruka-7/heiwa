use crate::schema::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use validator::{Validate, ValidationError};
use crate::services::database::establish_connection;

#[derive(Debug, Queryable, Identifiable, Selectable, PartialEq, Serialize)]
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

#[derive(Debug, Validate, Insertable, Deserialize)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    #[validate(custom = "validate_unique_permalink")]
    pub permalink: String,
    pub title: String,
    pub content: Option<String>,
    pub meta_description: Option<String>,
    pub author_id: i32,
}

impl Article {
    pub fn find_by_permalink(permalink_param: String) -> QueryResult<Vec<Article>> {
        Article::table()
            .filter(articles::permalink.eq(permalink_param))
            .limit(1)
            .select(Article::as_select())
            .load(&mut establish_connection())
    }

    pub fn find_by_author(author_param: i32) -> QueryResult<Vec<Article>> {
        Article::table()
            .filter(articles::author_id.eq(author_param))
            .select(Article::as_select())
            .load(&mut establish_connection())
    }

    pub fn find_by_tag(tag_id: i32) -> QueryResult<Vec<Article>> {
        articles::table
            .inner_join(articles_tags::table)
            .filter(tags::id.eq(tag_id))
            .select(Article::as_select())
            .load(&mut establish_connection())
    }

    pub fn create(new_article: NewArticle) -> QueryResult<Article> {
        insert_into(articles::table)
            .values(&new_article)
            .returning(Article::as_returning())
            .get_result(&mut establish_connection())
    }

    pub fn delete(id: i32) -> QueryResult<usize> {
        delete(Article::table().find(id))
            .execute(&mut establish_connection())
    }
}

pub fn validate_unique_permalink(name: &str) -> Result<(), ValidationError> {
    let article_result: QueryResult<Vec<Article>> = Article::find_by_permalink(name.to_string());
    match article_result {
        Ok(articles) => {
            if articles.is_empty() {
                Ok(())
            } else {
                Err(ValidationError::new("PERMALINK_EXIST"))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(ValidationError::new("VALIDATE_PERMALINK_ERROR"))
        }
    }
}
