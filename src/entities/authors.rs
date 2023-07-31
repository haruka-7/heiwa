use crate::schema::*;
use crate::services::authors::hash_password;
use crate::services::database::connection_pool;
use crate::AppState;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{delete, insert_into, update};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::{Validate, ValidationError};

#[derive(Debug, Queryable, Identifiable, Selectable, PartialEq, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub display_name: String,
    pub biography: Option<String>,
    pub role: Option<String>,
    pub password: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String,
    #[validate(email, custom = "validate_unique_email")]
    pub email: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Validate)]
#[diesel(table_name = authors)]
pub struct UpdateAuthor {
    pub id: i32,
    pub name: Option<String>,
    #[validate(email, custom = "validate_unique_email")]
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
}

impl Author {
    pub fn find_by_name(
        mut connection: PooledConnection<ConnectionManager<PgConnection>>,
        name_param: String,
    ) -> QueryResult<Vec<Self>> {
        Self::table()
            .filter(authors::name.eq(name_param))
            .limit(1)
            .select(Author::as_select())
            .load(&mut connection)
    }

    pub fn find_by_email(email_param: String) -> QueryResult<Vec<Self>> {
        Self::table()
            .filter(authors::email.eq(email_param))
            .limit(1)
            .select(Author::as_select())
            .load(&mut connection_pool().get().unwrap())
    }

    pub fn create(mut new_author: NewAuthor) -> QueryResult<Author> {
        new_author.password = hash_password(&new_author.password);
        insert_into(authors::table)
            .values(&new_author)
            .returning(Author::as_returning())
            .get_result(&mut connection_pool().get().unwrap())
    }

    pub fn update(mut update_author: UpdateAuthor) -> QueryResult<usize> {
        if update_author.password.is_some() {
            update_author.password = Option::from(hash_password(&update_author.password.unwrap()));
        }
        update(&update_author)
            .set(&update_author)
            .execute(&mut connection_pool().get().unwrap())
    }

    pub fn delete(author_id: i32) -> QueryResult<usize> {
        delete(Author::table().find(author_id)).execute(&mut connection_pool().get().unwrap())
    }
}

pub fn validate_unique_name(state: Arc<AppState>, name: &str) -> Result<(), ValidationError> {
    let author_result: QueryResult<Vec<Author>> =
        Author::find_by_name(state.db_connection.get().unwrap(), name.to_string());
    match author_result {
        Ok(authors) => {
            if authors.is_empty() {
                Ok(())
            } else {
                Err(ValidationError::new("NAME_EXIST"))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(ValidationError::new("VALIDATE_NAME_ERROR"))
        }
    }
}

pub fn validate_unique_email(email: &str) -> Result<(), ValidationError> {
    let author_result: QueryResult<Vec<Author>> = Author::find_by_email(email.to_string());
    match author_result {
        Ok(authors) => {
            if authors.is_empty() {
                Ok(())
            } else {
                Err(ValidationError::new("EMAIL_EXIST"))
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(ValidationError::new("VALIDATE_EMAIL_ERROR"))
        }
    }
}
