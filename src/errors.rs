use actix_web::{ResponseError, HttpResponse};
use diesel::result::{Error as DBError, DatabaseErrorKind};
use std::error::Error as STDError;
use serde::export::TryFrom;

pub type ValyouResult<T> = std::result::Result<T, Error>;

pub type RequestResult = ValyouResult<HttpResponse>;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Not Found")]
    NotFound,
}

#[derive(Debug, Display)]
pub enum ConstraintViolation {
    AuthorOwnsJournal,
    ProperEmail,
    EditAfterDay,
    EditTimestamp,
    ArePublic
}

impl STDError for Error {}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            Error::BadRequest(message) => HttpResponse::BadRequest().json(message),
            Error::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            Error::NotFound => HttpResponse::NotFound().finish()
        }
    }
}

impl<'a> From<DBError> for Error {
    fn from(error: DBError) -> Error {
        match error {
            DBError::DatabaseError(kind, info) => {
                match kind {
                    DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation => {
                        let message = info.details().unwrap_or_else(|| info.message()).to_string();
                        Error::BadRequest(message)
                    },
                    _ => {
                        if let Some(name) = info.constraint_name() {
                            if let Ok(cv) = ConstraintViolation::try_from(name) {
                                return Error::from(cv);
                            }
                        }

                        Error::InternalServerError
                    }
                }
            },
            DBError::NotFound => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(_: r2d2::Error) -> Self { Error::InternalServerError }
}

impl std::convert::TryFrom<&str> for ConstraintViolation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "author_owns_journal" => Ok(ConstraintViolation::AuthorOwnsJournal),
            "proper_email" => Ok(ConstraintViolation::ProperEmail),
            "edit_after_day" => Ok(ConstraintViolation::EditAfterDay),
            "edit_timestamp" => Ok(ConstraintViolation::EditTimestamp),
            "are_public" => Ok(ConstraintViolation::ArePublic),
            _ => Err(())
        }
    }
}

impl From<ConstraintViolation> for Error {
    fn from(cv: ConstraintViolation) -> Self {
        match cv {
            ConstraintViolation::AuthorOwnsJournal => Error::BadRequest("User does not own journal".into()),
            ConstraintViolation::ProperEmail => Error::BadRequest("Please provide a valid email address".into()),
            ConstraintViolation::EditAfterDay => Error::BadRequest("Cannot edit the content of an entry 24 hours after it's creation".into()),
            ConstraintViolation::EditTimestamp => Error::BadRequest("Cannot edit a timestamp".into()),
            ConstraintViolation::ArePublic => Error::BadRequest("You and the other person must have non-private profiles".into()),
        }
    }
}