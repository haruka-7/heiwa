use crate::entities::articles::Article;
use crate::entities::articles_tags::ArticleTag;
use crate::schema::*;
use crate::services::database::establish_connection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Queryable, Identifiable, Selectable, PartialEq, Serialize)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i32,
    pub permalink: String,
    pub label: String,
}

#[derive(Debug, Validate, Insertable, PartialEq, Deserialize)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTag {
    #[validate(custom = "validate_unique_permalink")]
    pub permalink: String,
    pub label: String,
}

impl Tag {
    pub fn find_tag_by_permalink(permalink_param: String) -> QueryResult<Vec<Tag>> {
        Tag::table()
            .filter(tags::permalink.eq(permalink_param))
            .limit(1)
            .select(Tag::as_select())
            .load(&mut establish_connection())
    }

    pub fn find_tags_by_article(article: &Article) -> QueryResult<Vec<Tag>> {
        ArticleTag::belonging_to(&article)
            .inner_join(tags::table)
            .select(Tag::as_select())
            .load(&mut establish_connection())
    }

    pub fn create(new_tag: NewTag) -> QueryResult<Tag> {
        insert_into(tags::table)
            .values(&new_tag)
            .returning(Tag::as_returning())
            .get_result(&mut establish_connection())
    }

    pub fn delete(id: i32) -> QueryResult<usize> {
        delete(Tag::table().find(id)).execute(&mut establish_connection())
    }
}

pub fn validate_unique_permalink(permalink: &str) -> Result<(), ValidationError> {
    let tags_result: QueryResult<Vec<Tag>> = Tag::find_tag_by_permalink(permalink.to_string());
    match tags_result {
        Ok(tags) => {
            if tags.is_empty() {
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
