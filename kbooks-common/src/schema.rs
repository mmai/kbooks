table! {
    books (id) {
        id -> Int4,
        user_id -> Int4,
        librarything_id -> Nullable<Text>,
        title -> Text,
        author_lf -> Text,
        author_code -> Text,
        isbn -> Text,
        publicationdate -> Text,
        rating -> Nullable<Int4>,
        language_main -> Text,
        language_secondary -> Nullable<Text>,
        language_original -> Text,
        review -> Nullable<Text>,
        cover -> Text,
        created_at -> Timestamp,
        dateacquired_stamp -> Nullable<Timestamp>,
        started_stamp -> Nullable<Timestamp>,
        finished_stamp -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        login -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        language -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    users,
);
