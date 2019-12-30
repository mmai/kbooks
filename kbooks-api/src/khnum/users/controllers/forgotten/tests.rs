use crate::khnum::users;
use actix_web::{web, test, http, App};
use actix_session::{CookieSession, Session};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use std::time::Duration;
use futures::future::Future;
use super::CommandResult;

use diesel::prelude::*;
use kbooks_common::khnum::schema::users::dsl;
use kbooks_common::khnum::users::models::{SlimUser, User, NewUser};
use kbooks_common::khnum::wiring::Config;

#[actix_rt::test]
async fn test_request() {
    dotenv().ok();
    let srv = test::start( || {
        let pool = kbooks_common::khnum::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let user = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), String::from("password"), String::from("fr_FR"));
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

        App::new().data(Config {pool: pool.clone(), front_url: String::from("http://dummy")}).service(
                                                                                                      web::resource("/user/forgotten").route(
                                                                                                          web::post().to(users::controllers::forgotten::request)
                                                                                                      )
        )
    });

    //==== Test request
    let form = super::RequestForm { email: String::from("email@toto.fr")};

    let req = srv.post("/user/forgotten")
        .timeout(Duration::new(15, 0));

    let mut response = req.send_form(&form).await.unwrap();
    println!("{:#?}", response);
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(result.success);

    //======== Test request with unknown email
    let unknown = super::RequestForm {
        email: String::from("email2@toto.fr"),
    };
    let req = srv.post("/user/forgotten")
        .timeout(Duration::new(15, 0));

    let mut response = req.send_form(&unknown).await.unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Email does not exists")), result.error);
}

use regex::Regex;

#[actix_rt::test]
async fn test_link() {
    dotenv().ok();
    let mut srv = test::start( move || {
        let pool = kbooks_common::khnum::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let user = NewUser::with_details(String::from("login"), String::from("email@test.fr"), String::from("password"), String::from("fr_FR"));
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

        App::new().data(Config {pool: pool.clone(), front_url: String::from("http://dummy")})
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service( web::resource("/user/forgotten").route( // To test insertions 
                    web::post().to(users::controllers::forgotten::request)
            ))
            .service( web::resource("/user/forgotten/{hashlink}/{email}/{expires_at}").route(
                    web::get().to(users::controllers::forgotten::check)
            ))
            .service( web::resource("/user/changepassword").route( 
                    web::post().to(users::controllers::forgotten::change_password),
                    ))
    });

    // ================ Good link
    let email = "email@test.fr";

    // 1. Mock request
    let req = forgotten_link_mock(&mut srv, email, email, false);
    // 2. Validate link
    let mut response = req.send().await.unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(result.success);
    // 3. Change password
    let mut req: awc::ClientRequest = srv.post("/user/changepassword").timeout(Duration::new(15, 0));
    req = keep_session(response, req);
    let form = super::PasswordForm { password: String::from("passwd") };
    let mut response = req.send_form(&form).await.unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    println!("{:?}", result);
    assert!(result.success);

    // ================ Bad link
    let emailbad = "emailo@test.fr";
    let req = forgotten_link_mock(&mut srv, email, emailbad, false);
    let mut response2 = req.send().await.unwrap();
    assert!(response2.status().is_success());
    let result: CommandResult = response2.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Incorrect link")), result.error);

    // ================ Link validity expired
    let req = forgotten_link_mock(&mut srv, email, email, true);
    let mut response = req.send().await.unwrap();
    // println!("response : {:#?}", response);
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Link validity expired")), result.error);
}

fn keep_session(response: awc::ClientResponse<impl futures::stream::Stream>, request: awc::ClientRequest) -> awc::ClientRequest {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"actix-session=([^;]*)"#).unwrap();
    }
    let cookies = response.headers().get("set-cookie").unwrap().to_str().unwrap();
    let cookie_session : &str = RE.captures(cookies).unwrap().get(1).unwrap().as_str();
    request.cookie(
        awc::http::Cookie::build("actix-session", format!("{}", cookie_session))
        .path("/").secure(false).finish(),
        )
}

fn forgotten_link_mock(srv: &mut actix_web::test::TestServer, email: &str, email_check: &str, expired: bool) -> awc::ClientRequest {
    let mut expires_at = super::Local::now().naive_local() + super::Duration::hours(24);
    if expired {
        expires_at = super::Local::now().naive_local() - super::Duration::hours(24);
    }
    let validate_params = format!("{}{}", email, expires_at.timestamp());
    let link = super::make_confirmation_data(&validate_params);
    let confirmation_hash = super::hash_password(&link)
        .map(|hash| super::to_url(&hash))
        .expect("Error hashing link");
    return srv.get(format!("/user/forgotten/{}/{}/{}", confirmation_hash, email_check, expires_at.timestamp()))
        .timeout(Duration::new(15, 0));
}
