// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    ip_records (ip) {
        ip -> Inet,
        banned -> Bool,
        fetch_cookie_date -> Date,
    }
}

diesel::table! {
    posts (id) {
        id -> Int8,
        user_id -> Varchar,
        content -> Text,
        flag -> Int8,
        create_time -> Timestamp,
        deleted -> Bool,
        thread_id -> Int4,
        thread_path -> Array<Nullable<Int8>>,
        email -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
    }
}

diesel::table! {
    threads (id) {
        id -> Int4,
        primary_post_id -> Int8,
        update_time -> Timestamp,
        board_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        flag -> Int8,
        create_time -> Timestamp,
        ban_time -> Nullable<Timestamp>,
        cookiehash -> Array<Nullable<Int4>>,
    }
}

diesel::joinable!(posts -> users (user_id));
diesel::joinable!(threads -> posts (primary_post_id));

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    ip_records,
    posts,
    threads,
    users,
);
