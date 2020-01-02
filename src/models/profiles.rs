use diesel::{Queryable, sql_types::*};

use crate::models::{self, visibility::{db, Visibility}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Username {
    pub username: String,
    #[serde(with = "models::discriminator_serde")]
    pub discriminator: i16
}

#[derive(Debug, Serialize)]
pub struct Profile {
    #[serde(with = "models::id_serde")]
    pub userid: i64,
    pub username: Username,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FullProfile {
    pub profile: Profile,
    pub visibility: Visibility,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_modified: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct Friend {
    pub with: Profile,
    pub since: chrono::NaiveDateTime
}

impl Profile {
    #[inline(always)]
    pub fn new(userid: i64, username: String, discriminator: i16, summary: Option<String>, bio: Option<String>) -> Self {
        Profile {
            userid,
            username: Username {
                username,
                discriminator
            },
            summary,
            bio
        }
    }
}

impl Queryable<(BigInt, Text, SmallInt, Nullable<Text>, Nullable<Text>, db::Visibility, Timestamp, Nullable<Timestamp>, Nullable<Timestamp>), diesel::pg::Pg> for FullProfile {
    type Row = (i64, String, i16, Option<String>, Option<String>, Visibility, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>);

    fn build(row: Self::Row) -> Self {
        FullProfile {
            profile: Profile::new(row.0, row.1, row.2, row.3, row.4),
            visibility: row.5,
            created: row.6,
            modified: row.7,
            username_modified: row.8
        }
    }
}

impl Queryable<(BigInt, Text, SmallInt, Nullable<Text>, Nullable<Text>, Timestamp), diesel::pg::Pg> for Friend {
    type Row = (i64, String, i16, Option<String>, Option<String>, chrono::NaiveDateTime);

    fn build(row: Self::Row) -> Self {
        Friend {
            with: Profile::new(row.0, row.1, row.2, row.3, row.4),
            since: row.5
        }
    }
}

impl Queryable<(BigInt, Text, SmallInt, Nullable<Text>, Nullable<Text>), diesel::pg::Pg> for Profile {
    type Row = (i64, String, i16, Option<String>, Option<String>);

    fn build(row: Self::Row) -> Self {
        Profile::new(row.0, row.1, row.2, row.3, row.4)
    }
}