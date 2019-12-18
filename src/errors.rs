use actix_web::{ResponseError, HttpResponse};
use diesel::result::{Error as DBError, DatabaseErrorKind};
use std::fmt::Display;
use std::error::Error;

pub type ValyouResult<T> = std::result::Result<T, ValyouError>;

pub type RequestResult = std::result::Result<HttpResponse, HttpResponse>;

#[derive(Debug, Display)]
pub enum ValyouError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl Error for ValyouError {

}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ValyouError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ValyouError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ValyouError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ValyouError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl From<DBError> for ValyouError {
    fn from(error: DBError) -> ValyouError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ValyouError::BadRequest(message);
                }
                ValyouError::InternalServerError
            }
            _ => ValyouError::InternalServerError,
        }
    }
}