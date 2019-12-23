use diesel::{sql_types::*, Queryable};

use crate::models::visibility::Visibility;

pub mod status;
pub mod visibility;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Account {
    pub id: i64,
    pub email: String,
    #[serde(skip)]
    pub hash: String,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Entry {
    pub id: i64,
    pub author: i64,
    pub journal: i64,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiedc: Option<chrono::NaiveDateTime>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub significance: Option<f64>
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Journal {
    pub id: i64,
    pub owner: i64,
    pub name: String,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub color: i32
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Profile {
    pub id: i64,
    pub summary: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Queryable, QueryableByName)]
pub struct Friend {
    #[sql_type = "Int8"]
    pub userid: i64,

    #[sql_type = "Varchar"]
    pub username: String,

    #[sql_type = "Int2"]
    pub discriminator: i16,

    #[sql_type = "Nullable<Varchar>"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[sql_type = "Nullable<Varchar>"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[sql_type = "Timestamp"]
    pub since: chrono::NaiveDateTime
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMethod {
    Around,
    Before,
    After
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub id: i64,
    #[serde(default = "default_limit")]
    pub limit: i64
}

#[inline(always)]
const fn default_limit() -> i64 { 20 }

