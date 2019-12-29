use diesel::{Queryable, sql_types::*};

use crate::models::{self, visibility::{db, Visibility}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(with = "models::id_serde")]
    pub id: i64,
    #[serde(with = "models::id_serde")]
    pub author: i64,
    #[serde(with = "models::id_serde")]
    pub journal: i64,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiedc: Option<chrono::NaiveDateTime>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub significance: Option<f64>,
    pub tags: Vec<String>
}

impl Queryable<(BigInt, BigInt, BigInt, Timestamp, Nullable<Timestamp>, Nullable<Timestamp>, Text, Nullable<Double>, Text), diesel::pg::Pg> for Entry {
    type Row = (i64, i64, i64, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, String, Option<f64>, String);

    fn build(row: Self::Row) -> Self {
        Entry {
            id: row.0,
            author: row.1,
            journal: row.2,
            created: row.3,
            modified: row.4,
            modifiedc: row.5,
            content: row.6,
            significance: row.7,
            tags: row.8.split(',').map(|s| s.into()).collect()
        }
    }
}

impl Queryable<(BigInt, BigInt, BigInt, Timestamp, Nullable<Timestamp>, Nullable<Timestamp>, Text, Nullable<Double>), diesel::pg::Pg> for Entry {
    type Row = (i64, i64, i64, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, String, Option<f64>);

    fn build(row: Self::Row) -> Self {
        Entry {
            id: row.0,
            author: row.1,
            journal: row.2,
            created: row.3,
            modified: row.4,
            modifiedc: row.5,
            content: row.6,
            significance: row.7,
            tags: Vec::with_capacity(0)
        }
    }
}