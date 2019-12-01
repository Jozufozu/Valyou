use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub username: String,
    pub passhash: String,
    pub email: String,
    pub joined: Timestamp,
    pub modified: Option<Timestamp>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
}