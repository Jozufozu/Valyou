use diesel::sql_types::Timestamp;
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgTypeMetadata};
use std::io::Write;
use diesel::{serialize, deserialize};

use crate::schema::{accounts, entries, profiles};
use crate::models::visibility::{Visibility, Publicity};

pub mod status;
pub mod visibility;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: i64,
    pub email: String,
    #[serde(skip)]
    pub hash: String,
    pub created: chrono::NaiveDateTime,
    pub modified: Option<chrono::NaiveDateTime>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "entries"]
pub struct Entry {
    pub id: i64,
    pub author: i64,
    pub visibility: Publicity,
    pub created: chrono::NaiveDateTime,
    pub modified: Option<chrono::NaiveDateTime>,
    pub modifiedc: Option<chrono::NaiveDateTime>,
    pub significance: Option<f64>,
    pub hidden: bool
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "profiles"]
pub struct Profile {
    pub id: i64,
    pub summary: Option<String>,
    pub bio: Option<String>,
}

