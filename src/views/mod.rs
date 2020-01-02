
table! {
    new_account (email) {
        email -> Varchar,
        hash -> Varchar,
        username -> Varchar,
    }
}

table! {
    searchable (userid) {
        userid -> Int8,
        username -> Varchar,
        discriminator -> SmallInt,
        summary -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
    }
}

table! {
    use crate::models::visibility::db::Visibility;
    use diesel::sql_types::*;

    full_profiles (userid) {
        userid -> Int8,
        username -> Varchar,
        discriminator -> SmallInt,
        summary -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        visibility -> Visibility,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        username_modified -> Nullable<Timestamp>,
    }
}

table! {
    public_friends (userid, friend) {
        userid -> Int8,
        friend -> Int8,
        username -> Varchar,
        discriminator -> SmallInt,
        summary -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        since -> Timestamp,
    }
}

table! {
    friend_requests (userid, friend) {
        userid -> Int8,
        friend -> Int8,
        username -> Varchar,
        discriminator -> SmallInt,
        summary -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        since -> Timestamp,
    }
}

table! {
    visible_entries (entryid) {
        entryid -> Int8,
        author -> Int8,
        journal -> Int8,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        modifiedc -> Nullable<Timestamp>,
        content -> Varchar,
        significance -> Nullable<Float8>,
        tags -> Varchar,
    }
}

table! {
    hidden_entries (entryid) {
        entryid -> Int8,
        author -> Int8,
        journal -> Int8,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
        modifiedc -> Nullable<Timestamp>,
        content -> Varchar,
        significance -> Nullable<Float8>,
        tags -> Varchar,
    }
}