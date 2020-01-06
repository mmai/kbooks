#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
//For tests
#[macro_use] extern crate diesel_migrations;

use actix::prelude::*;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_session::{CookieSession, Session};
// use chrono::Duration;
use diesel::{r2d2::ConnectionManager};
use dotenv::dotenv;

use actix_i18n::Translations;
use gettext_macros::{compile_i18n, include_i18n, init_i18n};

use kbooks_common::khnum::wiring;

init_i18n!("khnum", en, fr); // Put this before modules containing messages to be translated

mod khnum;
mod controllers;

// fn hello(
//     session: Session,
//     i18n: I18n
//     ) -> HttpResponse {
//          let msg_expire = i18n!(&i18n.catalog, "your Invitation expires on");
//          HttpResponse::Ok().json(msg_expire)
// }

// #[cfg_attr(tarpaulin, skip)]
// fn main() -> std::io::Result<()> {
//     actix_rt::System::new("kbooks").block_on(serve())
// }

#[cfg_attr(tarpaulin, skip)]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var( "RUST_LOG", "khnum=debug,actix_web=info,actix_server=info",);
    std::env::set_var("RUST_BACKTRACE", "1");//XXX works only for panic! macro
    env_logger::init();
    let front_url = dotenv::var("FRONT_URL").expect("FRONT_URL must be set");
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = wiring::db_init(db_url);

    HttpServer::new(move || {
        // secret is a random minimum 32 bytes long base 64 string
        let secret: String = dotenv::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
        // let domain: String = dotenv::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

        App::new()
            .data(wiring::Config { 
                pool: pool.clone(),
                front_url: front_url.clone()
            })
            .app_data(managed_state())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(secret.as_bytes()).secure(false))
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(secret.as_bytes())
            //         .name("auth")
            //         .path("/")
            //         .domain(domain.as_str())
            //         .max_age_time(Duration::days(1))
            //         .secure(false), // this can only be true if you have https
            // ))
            .service( web::scope("/api") // everything under '/api/' route
                    .service( web::resource("/auth") // routes for authentication
                            .route(web::post().to(khnum::users::controllers::auth::login))
                            .route(web::delete().to(khnum::users::controllers::auth::logout))
                            .route(web::get().to(khnum::users::controllers::auth::get_me)),
                    )
                    .service( web::resource("/book") // routes for authentication
                            .route(web::post().to(controllers::book::create))
                            .route( web::get().to(controllers::book::list))
                    )
            )
            .service( web::scope("/register") // everything under '/register/' route
                  .service( web::resource("/request").route(
                      web::post().to(khnum::users::controllers::register::request)
                  ))
                  // route to validate registration
                  .service( web::resource("/register/{hashlink}/{login}/{hpass}/{email}/{expires_at}").route(
                          web::get().to(khnum::users::controllers::register::register)
                  ))
                  // .service( web::resource("/validate").route(
                  //         web::post().to(khnum::users::controllers::register::register)
                  // ))
            )
            // .service( web::resource("/hello").route(
            //         web::get().to(hello)
            // ))
            // serve static files
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .start()
    .await
}

#[cfg_attr(tarpaulin, skip)]
pub fn managed_state() -> Translations {
    include_i18n!()
}

compile_i18n!();
