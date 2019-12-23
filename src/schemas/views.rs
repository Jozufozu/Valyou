
table! {
    use crate::models::{status::Status, visibility::db::Visibility};
    use diesel::sql_types::*;

    new_account (email) {
        email -> Varchar,
        hash -> Varchar,
        username -> Varchar,
    }
}