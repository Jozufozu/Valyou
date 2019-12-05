use diesel::{serialize, deserialize};
use diesel::deserialize::FromSql;
use diesel::serialize::{ToSql, IsNull, Output};
use diesel::pg::Pg;
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "visibility")]
pub struct Visibility;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Visibility"]
pub enum Publicity {
    Public,
    Private,
    Friends
}

impl ToSql<Visibility, Pg> for Publicity {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Publicity::Public => out.write_all(b"public")?,
            Publicity::Private => out.write_all(b"private")?,
            Publicity::Friends => out.write_all(b"friends")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Visibility, Pg> for Publicity {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"public" => Ok(Publicity::Public),
            b"private" => Ok(Publicity::Private),
            b"friends" => Ok(Publicity::Friends),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}