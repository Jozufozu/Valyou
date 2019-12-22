table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    accounts (id) {
        id -> Int8,
        email -> Varchar,
        hash -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        phone -> Nullable<Varchar>,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    entries (id) {
        id -> Int8,
        author -> Int8,
        journal -> Int8,
        visibility -> Visibility,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        modifiedc -> Nullable<Timestamp>,
        content -> Varchar,
        significance -> Nullable<Float8>,
        hidden -> Bool,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    entry_tags (entry, tag) {
        entry -> Int8,
        tag -> Varchar,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    journals (id) {
        id -> Int8,
        owner -> Int8,
        name -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        description -> Nullable<Varchar>,
        visibility -> Visibility,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    profiles (id) {
        id -> Int8,
        visibility -> Visibility,
        summary -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        modified -> Nullable<Timestamp>,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    relations (user_from, user_to) {
        user_from -> Int8,
        user_to -> Int8,
        since -> Timestamp,
        status -> Status,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    usernames (id) {
        id -> Int8,
        handle -> Varchar,
        modified -> Nullable<Timestamp>,
    }
}

joinable!(entries -> journals (journal));
joinable!(entries -> profiles (author));
joinable!(entry_tags -> entries (entry));
joinable!(journals -> profiles (owner));
joinable!(profiles -> accounts (id));
joinable!(usernames -> profiles (id));

allow_tables_to_appear_in_same_query!(
    accounts,
    entries,
    entry_tags,
    journals,
    profiles,
    relations,
    usernames,
);
