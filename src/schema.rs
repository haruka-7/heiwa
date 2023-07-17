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

diesel::table! {
    articles (id) {
        id -> Int4,
        permalink -> Varchar,
        title -> Varchar,
        creation_date -> Timestamp,
        publication_date -> Nullable<Timestamp>,
        update_date -> Nullable<Timestamp>,
        content -> Nullable<Text>,
        published -> Bool,
        meta_description -> Nullable<Varchar>,
        author_id -> Int4,
    }
}

diesel::table! {
    articles_tags (article_id, tag_id) {
        article_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        permalink -> Varchar,
        label -> Varchar,
    }
}

diesel::joinable!(links -> authors (author_id));
diesel::joinable!(articles_tags -> articles (article_id));
diesel::joinable!(articles_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(authors, links,);

diesel::allow_tables_to_appear_in_same_query!(articles, articles_tags, tags,);
