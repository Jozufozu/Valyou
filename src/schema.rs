table! {
    accounts (id) {
        id -> Int8,
        username -> Varchar,
        passhash -> Varchar,
        email -> Varchar,
        joined -> Timestamp,
        modified -> Nullable<Timestamp>,
        name -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
    }
}
