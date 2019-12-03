use diesel::sql_types::Timestamp;
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgTypeMetadata};
use std::io::Write;
use diesel::{serialize, deserialize};

#[derive(SqlType)]
#[postgres(type_name = "status")]
pub struct Status;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Status"]
pub enum RelationStatus {
    PendingFirstSecond,
    PendingSecondFirst,
    Friends,
    BlockFirstSecond,
    BlockSecondFirst,
    BlockBoth
}

impl ToSql<Status, Pg> for RelationStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            RelationStatus::PendingFirstSecond => out.write_all(b"pending_first_second")?,
            RelationStatus::PendingSecondFirst => out.write_all(b"pending_second_first")?,
            RelationStatus::Friends => out.write_all(b"friends")?,
            RelationStatus::BlockFirstSecond => out.write_all(b"block_first_second")?,
            RelationStatus::BlockSecondFirst => out.write_all(b"block_second_first")?,
            RelationStatus::BlockBoth => out.write_all(b"block_both")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Status, Pg> for RelationStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"pending_first_second" => Ok(RelationStatus::PendingFirstSecond),
            b"pending_second_first" => Ok(RelationStatus::PendingSecondFirst),
            b"friends" => Ok(RelationStatus::Friends),
            b"block_first_second" => Ok(RelationStatus::BlockFirstSecond),
            b"block_second_first" => Ok(RelationStatus::BlockSecondFirst),
            b"block_both" => Ok(RelationStatus::BlockBoth),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

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