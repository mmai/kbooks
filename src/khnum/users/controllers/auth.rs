use actix_identity::Identity;
use actix_session::{CookieSession, Session};
use actix_web::{web, Error, error::BlockingError, HttpRequest, HttpResponse, Responder, ResponseError};

use futures::future::Future;

use crate::khnum::wiring::{DbPool, Config};
use crate::khnum::errors::ServiceError;

use crate::khnum::users::repository::auth_handler;
// use crate::khnum::users::utils::create_token; //for JWT token
use crate::khnum::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    login: String,
    password: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CommandResult {
//     success: bool,
//     error: Option<String>
// }

pub fn login(
    auth_data: web::Form<AuthData>,
    session: Session,
    id: Identity,
    config: web::Data<Config>,
    ) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let data: AuthData = auth_data.into_inner();

    web::block( move || auth_handler::auth(config.pool.clone(), data.login, data.password))
        .then(move |res| { 
            match res {
            Ok(user) => {
                //Via jwt
                // let token = create_token(&user)?;
                // id.remember(token);
                //Via session cookie
                session.set("user", &user);
                Ok(HttpResponse::Ok().json(models::FrontUser::from(user)))
                // if session.set("user", &user).is_ok() {
                //     Ok(HttpResponse::Ok().json(user))
                // }
                // Ok(err.error_response())
            }
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                _ => Err(ServiceError::InternalServerError),
            },
            // Err(err) => {
            //     // panic!(" the error : {:?}", err); //XXX Is this the only way to show the error ?
            //     Err(err.into())
            //     BlockingError::Error(service_error) => Err(service_error),
            //     // Ok(err.error_response())
            // }
        }})
}

pub fn logout( session: Session, id: Identity) -> impl Responder {
    session.clear();
    id.forget();
    HttpResponse::Ok()
}

pub fn get_me(
    session: Session,
    // logged_user: auth_handler::LoggedUser
    ) -> HttpResponse {
    // ) -> impl Future<Item = HttpResponse, Error = Error> {
        let opt = session.get::<models::User>("user").expect("could not get session user");
        match opt {
            // Ok(user) => Ok(HttpResponse::Ok().json(user)),
            // Err(err) => Ok(err.error_response())
            Some(user) => HttpResponse::Ok().json(models::FrontUser::from(user)),
            None => HttpResponse::Unauthorized().json("Unauthorized")
        }
        // let user = opt.unwrap();
    // HttpResponse::Ok().json(logged_user)
}

#[cfg(test)]
mod tests;
