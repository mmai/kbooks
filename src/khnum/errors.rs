use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as UuidError;
use actix::MailboxError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try later"),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            ServiceError::Unauthorized(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }
        }
    }
}

impl From<MailboxError> for ServiceError {
    fn from(_: MailboxError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<UuidError> for ServiceError {
    fn from(_: UuidError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(_kind, info) => {
                // if let DatabaseErrorKind::UniqueViolation = kind {
                    let message =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                // }
                // ServiceError::InternalServerError
            }
            _ => {
                // println!("debug: default error {:?}", error);
                ServiceError::InternalServerError
            },
        }
    }
}
