use crate::entities::authors::Author;
use crate::entities::links::{Link, NewLink};
use crate::schema::links;
use diesel::associations::HasTable;
use diesel::{
    delete, insert_into, pg::PgConnection, BelongingToDsl, QueryDsl, QueryResult, RunQueryDsl,
    SelectableHelper,
};

