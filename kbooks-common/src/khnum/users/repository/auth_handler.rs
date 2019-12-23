// use actix::{Handler, Message};
// use actix_web::{web, dev::Payload, Error, HttpRequest};
// use actix_web::FromRequest;
// use actix_identity::Identity;
use bcrypt::verify;
use diesel::prelude::*;

use crate::khnum::wiring::{DbPool, Config};

use crate::khnum::errors::ServiceError;
use crate::khnum::users::models::{SlimUser, User};
// use crate::khnum::users::utils::decode_token; //for JWT token
use crate::khnum::wiring::MyConnection;

pub fn auth(pool: DbPool, login: String, password: String) -> Result<User, ServiceError> {
    use crate::khnum::schema::users::dsl;
    let conn: &MyConnection = &pool.get().unwrap();

    let mut items = dsl::users.filter(dsl::login.eq(&login)).load::<User>(conn)?;

    if let Some(user) = items.pop() {
        match verify(&password, &user.password) {
            Ok(matching) => {
                if matching {
                    return Ok(user);
                    // return Ok(user.into());
                }
            }
            Err(_) => ()
        }
    }
    Err(ServiceError::Unauthorized(
            "Username and Password don't match".into(),
            ))
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
// pub type LoggedUser = SlimUser;
//
// impl FromRequest for LoggedUser {
//     type Config = ();
//     type Error = Error;
//     type Future = Result<LoggedUser, Error>;
//
//     fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
//         if let Some(identity) = Identity::from_request(req, pl)?.identity() {
//             let user: SlimUser = decode_token(&identity)?;
//             return Ok(user as LoggedUser);
//         }
//         Err(ServiceError::Unauthorized("".into()).into())
//     }
// }
