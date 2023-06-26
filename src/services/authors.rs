use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::entities::authors::{Author, NewAuthor};
use crate::schema::authors;
use crate::schema::authors::dsl::authors as dsl_authors;
use diesel::{
    delete, insert_into, pg::PgConnection, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use crate::utils::establish_connection;
use validator::ValidationError;

pub fn get_authors(connection: &mut PgConnection) -> Vec<Author> {
    dsl_authors
        .select(Author::as_select())
        .load(connection)
        .expect("Should load handlers")
}

pub fn get_authors_by_name(connection: &mut PgConnection, name_param: String) -> Vec<Author> {
    dsl_authors
        .filter(authors::name.eq(name_param))
        .limit(1)
        .select(Author::as_select())
        .load(connection)
        .expect("Should load handlers")
}

pub fn create_author(connection: &mut PgConnection, mut new_author: NewAuthor) -> Author {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2: Argon2 = Argon2::default();
    new_author.password = argon2.hash_password(new_author.password.as_ref(), &salt).unwrap().to_string();
    insert_into(authors::table)
        .values(&new_author)
        .returning(Author::as_returning())
        .get_result(connection)
        .expect("Should save new author")
}

pub fn delete_author(connection: &mut PgConnection, id: i32) -> usize {
    delete(dsl_authors.find(id))
        .execute(connection)
        .expect("Should delete author")
}

pub fn validate_unique_name(name: &str) -> Result<(), ValidationError> {
    match get_authors_by_name(&mut establish_connection(), name.to_string()).first() {
        None => Ok(()),
        Some(_) => Err(ValidationError::new("name already taken")),
    }
}
