use diesel::{Queryable, sql_types::*};

use crate::models::visibility::Visibility;

pub mod status;
pub mod visibility;
pub mod profiles;
pub mod entries;
pub mod search;

sql_function! {
    fn can_see_entry(me: Bigint, author: Bigint, journal: Bigint) -> Bool;
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Account {
    #[serde(with = "id_serde")]
    pub id: i64,
    pub email: String,
    #[serde(skip)]
    pub hash: String,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_hash: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Journal {
    #[serde(with = "id_serde")]
    pub id: i64,
    #[serde(with = "id_serde")]
    pub owner: i64,
    pub name: String,
    pub created: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub color: i32
}

pub mod id_serde {
    use std::fmt;

    use serde::{de, Deserializer, ser, Serializer};
    use serde::de::Visitor;

    pub fn serialize<S>(val: &i64, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if *val < 0 {
            Err(ser::Error::custom(format!("expected value > 0")))
        } else {
            ser.serialize_str(format!("{:019}", val).as_str())
        }
    }

    struct Vis;

    impl<'de> Visitor<'de> for Vis {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string with integer value between \"0\" and 2^63")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            if value.len() > 19 {
                Err(E::custom(format!("expected string with max length 19")))
            } else {
                value.parse::<i64>().map_err(E::custom)
            }
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<i64, D::Error>
        where D: Deserializer<'de> {
        de.deserialize_str(Vis)
    }
}

pub mod discriminator_serde {
    use std::fmt;

    use serde::{de, Deserializer, ser, Serializer};
    use serde::de::Visitor;

    pub fn serialize<S>(val: &i16, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        if *val > 9999 || *val < 0 {
            Err(ser::Error::custom(format!("expected value to be within [0, 10000)")))
        } else {
            ser.serialize_str(format!("{:04}", val).as_str())
        }
    }

    struct Vis;

    impl<'de> Visitor<'de> for Vis {
        type Value = i16;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string with integer value between \"0000\" and \"9999\"")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            if value.len() != 4 {
                Err(E::custom(format!("expected string of length 4")))
            } else {
                value.parse::<i16>().map_err(de::Error::custom)
            }
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<i16, D::Error>
        where D: Deserializer<'de> {
        de.deserialize_str(Vis)
    }
}