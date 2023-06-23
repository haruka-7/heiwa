// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        display_name -> Varchar,
        password -> Varchar,
        biography -> Nullable<Text>,
        role -> Nullable<Varchar>,
    }
}

diesel::table! {
    links (id) {
        id -> Int4,
        url -> Varchar,
        title -> Varchar,
        author_id -> Int4,
    }
}

diesel::joinable!(links -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(authors, links,);
