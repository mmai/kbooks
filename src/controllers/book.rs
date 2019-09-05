use actix_session::{Session};
use actix_web::{ test, web, Error, error, HttpResponse, ResponseError, http};
use chrono::{Duration, Local, NaiveDateTime};
use futures::future::{Future, result, err};

use crate::khnum::wiring::{DbPool, Config, make_front_url};
use crate::khnum::errors::ServiceError;

use crate::repository::book_handler;
use crate::models::{Book, NewBook};

use actix_i18n::I18n;
use gettext::Catalog;
use gettext_macros::i18n;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    success: bool,
    error: Option<String>
}

// ---------------- Create Action------------

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBookForm {
    title: String,
}

pub fn create(
    form_data: web::Form<NewBookForm>,
    config: web::Data<Config>,
    i18n: I18n
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let form_data = form_data.into_inner();
    let res = check_existence(config.pool.clone(), &form_data.email, &form_data.username);
    match res {
        Ok(cde_res) => {
            if !cde_res.success {
                result(Ok(HttpResponse::Ok().json(cde_res)))
            } else {
                let hashed_password = hash_password(&form_data.password).expect("Error hashing password");
                let expires_at = Local::now().naive_local() + Duration::hours(24);
                // panic!(" avant send_confirmation");
                let res = send_confirmation(&i18n.catalog, form_data.username, hashed_password, form_data.email, expires_at);
                result(Ok(HttpResponse::Ok().json(res)))
            }
        }
        Err(err) => {
           result(Err(err))
        }
    }
}

#[cfg(test)]
mod tests;
// #[path = "./book_test.rs"] // avoid creating a /register folder
// mod book_test;
