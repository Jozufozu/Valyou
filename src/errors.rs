use actix_web::{ResponseError, HttpResponse};
use diesel::result::{Error as DBError, DatabaseErrorKind};
use std::fmt::Display;
use std::error::Error as STDError;

pub type ValyouResult<T> = std::result::Result<T, Error>;

pub type RequestResult = std::result::Result<HttpResponse, HttpResponse>;

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

impl STDError for Error {

}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Error::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            Error::NotFound => HttpResponse::NotFound().finish()
        }
    }
}

impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::InternalServerError
            },
            DBError::NotFound => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(_: r2d2::Error) -> Self { Error::InternalServerError }
}