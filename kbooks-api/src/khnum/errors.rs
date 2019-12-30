// use kbooks_common::khnum::errors::ServiceError;
// use actix_web::{error::ResponseError, HttpResponse};
// use actix::MailboxError;
//
// // impl ResponseError trait allows to convert our errors into http responses with appropriate data
// impl ResponseError for ServiceError {
//     fn error_response(&self) -> HttpResponse {
//         match *self {
//             ServiceError::InternalServerError => HttpResponse::InternalServerError()
//                 .json("Internal Server Error, Please try later"),
//             ServiceError::BadRequest(ref message) => {
//                 HttpResponse::BadRequest().json(message)
//             }
//             ServiceError::Unauthorized(ref message) => {
//                 HttpResponse::Unauthorized().json(message)
//             }
//         }
//     }
// }
//
// impl From<MailboxError> for ServiceError {
//     fn from(_: MailboxError) -> ServiceError {
//         ServiceError::InternalServerError
//     }
// }
//
