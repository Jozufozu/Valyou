table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    accounts (userid) {
        userid -> Int8,
        email -> Varchar,
        hash -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    entries (entryid) {
        entryid -> Int8,
        author -> Int8,
        journal -> Int8,
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

    journals (journalid) {
        journalid -> Int8,
        owner -> Int8,
        title -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        description -> Nullable<Varchar>,
        visibility -> Visibility,
        color -> Int4,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    profiles (userid) {
        userid -> Int8,
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
        status -> Status,
        since -> Timestamp,
    }
}

table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    usernames (userid) {
        userid -> Int8,
        username -> Varchar,
        discriminator -> Int2,
        modified -> Nullable<Timestamp>,
    }
}

joinable!(entries -> journals (journal));
joinable!(entries -> profiles (author));
joinable!(entry_tags -> entries (entry));
joinable!(journals -> profiles (owner));
joinable!(profiles -> accounts (userid));
joinable!(usernames -> profiles (userid));

allow_tables_to_appear_in_same_query!(
    accounts,
    entries,
    entry_tags,
    journals,
    profiles,
    relations,
    usernames,
);
