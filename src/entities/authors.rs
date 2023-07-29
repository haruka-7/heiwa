use crate::schema::*;
use crate::services::database::establish_connection;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use serde::{Deserialize, Serialize};
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
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    #[validate(custom = "validate_unique_name")]
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
    #[validate(custom = "validate_unique_name")]
    pub name: Option<String>,
    #[validate(email, custom = "validate_unique_email")]
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginAuthorPassword {
    pub name: String,
    pub role: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAuthor {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorAccessToken {
    pub access_token: String,
}

impl Author {
    pub fn find_by_name(name_param: String) -> QueryResult<Vec<Author>> {
        Author::table()
            .filter(authors::name.eq(name_param))
            .limit(1)
            .select(Author::as_select())
            .load(&mut establish_connection())
    }

    pub fn find_by_email(email_param: String) -> QueryResult<Vec<Author>> {
        Author::table()
            .filter(authors::email.eq(email_param))
            .limit(1)
            .select(Author::as_select())
            .load(&mut establish_connection())
    }

    pub fn create(mut new_author: NewAuthor) -> QueryResult<LoginAuthorPassword> {
        new_author.password = hash_password(&new_author.password);
        insert_into(authors::table)
            .values(&new_author)
            .returning(LoginAuthorPassword::as_returning())
            .get_result(&mut establish_connection())
    }

    pub fn update(mut update_author: UpdateAuthor) -> QueryResult<usize> {
        if update_author.password.is_some() {
            update_author.password = Option::from(hash_password(&update_author.password.unwrap()));
        }
        update(&update_author)
            .set(&update_author)
            .execute(&mut establish_connection())
    }

    pub fn delete(author_id: i32) -> QueryResult<usize> {
        delete(Author::table().find(author_id)).execute(&mut establish_connection())
    }
}

impl LoginAuthorPassword {
    pub fn find_by_name_for_login(name_param: String) -> QueryResult<Vec<LoginAuthorPassword>> {
        Author::table()
            .filter(authors::name.eq(name_param))
            .limit(1)
            .select(LoginAuthorPassword::as_select())
            .load(&mut establish_connection())
    }
}

pub fn validate_unique_name(name: &str) -> Result<(), ValidationError> {
    let author_result: QueryResult<Vec<Author>> = Author::find_by_name(name.to_string());
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

pub fn hash_password(password: &String) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2: Argon2 = Argon2::default();
    argon2
        .hash_password(password.as_ref(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(
    password: &String,
    password_hash: &str,
) -> argon2::password_hash::Result<()> {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Argon2::default().verify_password(password.as_ref(), &parsed_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password: String = "thee michel gun elephant".to_string();
        let password_hash: String = hash_password(&password);
        assert!(verify_password(&password, password_hash.as_ref()))
    }
}
