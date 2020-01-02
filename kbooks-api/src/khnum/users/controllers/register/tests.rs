use crate::khnum::users;
use actix_web::{web, test, http, App};
use actix_http::HttpService;
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

use actix_i18n::Translations;
use gettext_macros::include_i18n;

use std::fs::File;
use std::io::prelude::*;

fn error_log(mess: &str) -> std::io::Result<()> {
    let mut logfile = File::create("/tmp/kbooks_test_error.log")?;
    logfile.write(mess.as_bytes())?;
    Ok(())
}

pub fn managed_state() -> Translations {
    include_i18n!()
}

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

        App::new()
            .data(managed_state())
            .data(Config {pool: pool.clone(), front_url: String::from("http://dummy")}).service(
                                                                                                web::scope("/register") // everything under '/register/' route
                                                                                                .service( web::resource("/request").route(
                                                                                                        web::post().to(users::controllers::register::request)
                                                                                                )
                                                                                                )
            )
    });

    //==== Test request
    let form = super::RequestForm { 
        email:  String::from("email2@toto.fr"),
        username: String::from("toto"),
        password: String::from("totop")
    };

    error_log("test register request");
    let req = srv.post("/register/request")
        .timeout(Duration::new(15, 0));
        // .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

    let mut response = req.send_form(&form).await.unwrap();
    if (!response.status().is_success()){
        println!("{:#?}", response);
        println!("-- body: {:?}", response.body().await);
    }
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(result.success);

    //======== Test request with already registered email
    let existing_user = super::RequestForm {
        email: String::from("email@toto.fr"),
        username: String::from("toto"),
        password: String::from("totop")
    };
    let req = srv.post("/register/request")
        .timeout(Duration::new(15, 0));

    let mut response = req.send_form(&existing_user).await.unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Email already taken")), result.error);
}

use regex::Regex;

#[actix_rt::test]
async fn test_validate() {
    dotenv().ok();
    let mut srv = test::start( move || {
        let pool = kbooks_common::khnum::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let user = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), String::from("password"), String::from("fr_FR"));
        // Batch don't work with Sqlite 
        // diesel::insert_into(dsl::users).values(&vec![user, user_expired])
            // .execute(conn).expect("Error populating test database");
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

        App::new()
            .data(managed_state())
            .data(Config {pool: pool.clone(), front_url: String::from("http://dummy")})
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service( web::resource("/register/request").route( // To test insertions 
                    web::post().to(users::controllers::register::request)
            ))
            // .service( web::resource("/register/{hashlink}/{login}/{expires_at}/{register_url}").route(
            .service( web::resource("/register/register/{hashlink}/{login}/{hpass}/{email}/{expires_at}").route(
                    web::get().to(users::controllers::register::register)
            ))
    });

    // ================ Good link
    //
    let email = "email@test.fr";
    let username = "username";
    let passwd = "passwd";

    // 1. Mock register request
    let req = register_link_mock(&mut srv, username, passwd, email, email, false);
    // 2. Validate link and finish registration
    let mut response = req.send().await.unwrap();
    // assert!(response.status().is_redirection());
    // 3. Finish registration with user data
    // let mut req: awc::ClientRequest = srv.post("/register/validate").timeout(Duration::new(15, 0));
    // req = keep_session(response, req);
    // let form = super::ValidateForm { username:  String::from("username"), password: String::from("passwd") };
    // let mut response = req.send_form(&form).await.unwrap();
    // println!("{:?}", response);
    // println!("{:?}", response.status());
    assert!(response.status().is_redirection());
    // let result: CommandResult = response.json().await.expect("Could not parse json"); 
    // println!("{:?}", result);
    // assert!(result.success);

    // ----------- Registering with same email should now fail
    let form_request = super::RequestForm { 
        email:  String::from(email),
        username: String::from("toto"),
        password: String::from("totop")
    };
    let req_request = srv.post("/register/request").timeout(Duration::new(15, 0));
    let mut response = req_request.send_form(&form_request).await.unwrap();
    println!("response : {:#?}", response);
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Email already taken")), result.error);

    // ----------- Registering with same username should now fail
    // 1. Mock register request
    let email = "emailNewName@test.fr";
    let req = register_link_mock(&mut srv, username, passwd, email, email, false);
    // 2. Validate link
    let mut response = req.send().await.unwrap();
    // 3. Finish registration with user data
    // let mut req: awc::ClientRequest = srv.post("/register/validate").timeout(Duration::new(15, 0));
    // req = keep_session(response, req);
    // let form = super::ValidateForm { username:  String::from("username"), password: String::from("passwd") };
    // let mut response = req.send_form(&form).await.unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Username already taken")), result.error);

    // ================ Bad link
    //
    let emailbad = "emailo@test.fr";
    let username = "toto";
    let req = register_link_mock(&mut srv, username, passwd, email, emailbad, false);
    let mut response2 = req.send().await.unwrap();
    println!("{:?}", response2.status());
    assert!(response2.status().is_success());
    let result: CommandResult = response2.json().await.expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Incorrect link")), result.error);

    // ================ Link validity expired
    //
    let req = register_link_mock(&mut srv, username, passwd, email, email, true);
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

fn register_link_mock(srv: &mut actix_web::test::TestServer, username: &str, passwd: &str, email: &str, email_check: &str, expired: bool) -> awc::ClientRequest {
// fn register_link_mock(srv: &mut actix_http_test::TestServerRuntime, username: &str, passwd: &str, email: &str, email_check: &str, expired: bool) -> awc::ClientRequest {
    let mut expires_at = super::Local::now().naive_local() + super::Duration::hours(24);
    if expired {
        expires_at = super::Local::now().naive_local() - super::Duration::hours(24);
    }
    let hpasswd = super::hash_password(&passwd)
        .expect("Error hashing pwd");
    let base_url = "";
    let mut url = super::make_register_link(&base_url.to_string(), &username.to_string(), &hpasswd.to_string(), &email.to_string(), expires_at.timestamp());
    url = url.replace(email, email_check);
    return srv.get(url).timeout(Duration::new(15, 0));
}
