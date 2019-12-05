table! {
    use crate::models::{status::Status, visibility::Visibility};
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
    use crate::models::{status::Status, visibility::Visibility};
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
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    entry_tags (id) {
        id -> Int4,
        entry -> Nullable<Int8>,
        tag -> Varchar,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    feedback (id) {
        id -> Int8,
        author -> Int8,
        entry -> Int8,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        content -> Varchar,
        starred -> Bool,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    feedback_replies (child) {
        child -> Int8,
        parent -> Int8,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    journals (id) {
        id -> Int8,
        owner -> Int8,
        name -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        description -> Nullable<Varchar>,
        visibility -> Nullable<Visibility>,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    profiles (id) {
        id -> Int8,
        summary -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    relations (id) {
        id -> Int4,
        user_from -> Int8,
        user_to -> Int8,
        since -> Timestamp,
        status -> Status,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    user_visibility (id) {
        id -> Int8,
        visibility -> Visibility,
    }
}

table! {
    use crate::models::{status::Status, visibility::Visibility};
    use diesel::sql_types::*;

    usernames (id) {
        id -> Int8,
        handle -> Varchar,
    }
}

joinable!(entries -> journals (journal));
joinable!(entries -> profiles (author));
joinable!(entry_tags -> entries (entry));
joinable!(feedback -> entries (entry));
joinable!(feedback -> profiles (author));
joinable!(journals -> profiles (owner));

allow_tables_to_appear_in_same_query!(
    accounts,
    entries,
    entry_tags,
    feedback,
    feedback_replies,
    journals,
    profiles,
    relations,
    user_visibility,
    usernames,
);
