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
pub struct OwnProfile {
    pub profile: Profile,
    pub visibility: Visibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umodified: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct Friend {
    pub with: Profile,
    pub since: chrono::NaiveDateTime
}

impl Profile {
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

impl Queryable<(BigInt, Text, SmallInt, Nullable<Text>, Nullable<Text>, db::Visibility, Nullable<Timestamp>, Nullable<Timestamp>), diesel::pg::Pg> for OwnProfile {
    type Row = (i64, String, i16, Option<String>, Option<String>, Visibility, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>);

    fn build(row: Self::Row) -> Self {
        OwnProfile {
            profile: Profile::new(row.0, row.1, row.2, row.3, row.4),
            visibility: row.5,
            modified: row.6,
            umodified: row.7
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