use std::io::Write;

use diesel::{deserialize, serialize};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};

pub mod db {
    #[derive(SqlType, QueryId)]
    #[postgres(type_name = "visibility")]
    pub struct Visibility;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "db::Visibility"]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Friends
}

impl ToSql<db::Visibility, Pg> for Visibility {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Visibility::Public => out.write_all(b"public")?,
            Visibility::Private => out.write_all(b"private")?,
            Visibility::Friends => out.write_all(b"friends")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<db::Visibility, Pg> for Visibility {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"public" => Ok(Visibility::Public),
            b"private" => Ok(Visibility::Private),
            b"friends" => Ok(Visibility::Friends),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}