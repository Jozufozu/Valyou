use diesel::sql_types::Timestamp;
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgTypeMetadata};
use std::io::Write;
use diesel::{serialize, deserialize};

use crate::schema::accounts;

mod status;
mod visibility;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: u64,
    pub email: String,
    #[serde(skip)]
    pub hash: String,
    pub created: chrono::NaiveDateTime,
    pub modified: Option<chrono::NaiveDateTime>,
    pub phone: Option<String>,
}
