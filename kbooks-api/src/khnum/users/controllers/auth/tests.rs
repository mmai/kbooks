use crate::khnum::users;
use actix_web::{web, test, http, App};
use actix_session::{CookieSession, Session};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use std::time::Duration;
use futures::future::Future;

use diesel::prelude::*;
use crate::khnum::schema::users::dsl;
use crate::khnum::users::models::{FrontUser, User, NewUser};
use crate::khnum::users::utils::{hash_password};
use crate::khnum::wiring::Config;

#[actix_rt::test]
async fn test_login() {
    dotenv().ok();
    let srv = test::start(|| {
        let pool = crate::khnum::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let hashed_password = hash_password("password").expect("Error hashing password");
        let user = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), hashed_password, String::from("fr_FR"));
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

            App::new().data(Config {pool: pool.clone(), front_url: String::from("http://dummy")})
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service( web::resource("/auth") // routes for authentication
                      .route(web::post().to(users::controllers::auth::login))
                      .route(web::delete().to(users::controllers::auth::logout))
                      .route(web::get().to(users::controllers::auth::get_me)),
                      )
            // .service( web::resource("/login").route(
            //         web::post().to(users::controllers::auth::login)
            // ))
            // .service( web::resource("/me").route(
            //         web::post().to(users::controllers::auth::get_me)
            // ))
    });

    //==== Test login
    // let hashed_password = hash_password("password").expect("Error hashing password");
    let password = String::from("password");
    let form = super::AuthData { login: String::from("login"), password: password};
    let req = srv.post("/auth")
        // .header(http::header::CONTENT_TYPE, "application/json") // pour version send_body
        .timeout(Duration::new(15, 0));
    let mut response = req.send_form(&form).await.unwrap();
    assert!(response.status().is_success());
    let user: FrontUser = response.json().await.expect("Could not parse json"); 
    assert_eq!(user.email, String::from("email@toto.fr"));
    let parse_user: Result<User, awc::error::JsonPayloadError> = response.json().await;
    assert!(parse_user.is_err());
    // let result: CommandResult = response.json().await.expect("Could not parse json"); 
    // assert!(result.success);
    //should get user email
    let mut req = srv.get("/auth").timeout(Duration::new(15, 0));
    req = keep_session(response, req); //Via session cookie
    let mut response = req.send().await.unwrap();
    // println!("get me : {:#?}", response);
    assert!(response.status().is_success());
    let user: FrontUser = response.json().await.expect("Could not parse json"); 
    assert_eq!(user.email, String::from("email@toto.fr"));

    //======== Test request with bad password
    let bad = super::AuthData {
        login: String::from("login"),
        password: String::from("bad"),
    };
    let req = srv.post("/auth").timeout(Duration::new(15, 0));
    let response = req.send_form(&bad).await;
    println!(" bad : {:#?}", response);
    // assert!(!response.unwrap().status().is_success());
    assert_eq!("401", response.unwrap().status().as_str());

    //======== Test request with unknown login
    let unknown = super::AuthData {
        login: String::from("unknown"),
        password: String::from("unknown"),
    };
    let req = srv.post("/auth").timeout(Duration::new(15, 0));

    println!(" unknown get..");
    let response = req.send_form(&unknown).await;
    println!(" unknown : {:#?}", response);
    assert!(!response.unwrap().status().is_success());
    // let result: CommandResult = response.json().await.expect("Could not parse json"); 
    // assert!(!result.success);
    // assert_eq!(Some(String::from("Login does not exists")), result.error);
}

use regex::Regex;

#[actix_rt::test]
async fn test_logout() {
    dotenv().ok();
    let srv = test::start(|| {
        let pool = crate::khnum::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let hashed_password = hash_password("password").expect("Error hashing password");
        let user = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), hashed_password, String::from("fr_FR"));
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

            App::new().data(Config {pool: pool.clone(), front_url: String::from("http://dummy")})
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service( web::resource("/auth") // routes for authentication
                      .route(web::post().to(users::controllers::auth::login))
                      .route(web::delete().to(users::controllers::auth::logout))
                      .route(web::get().to(users::controllers::auth::get_me)),
                      )
            // .service( web::resource("/login").route(
            //         web::post().to(users::controllers::auth::login)
            // ))
            // .service( web::resource("/logout").route(
            //         web::get().to(users::controllers::auth::logout)
            // ))
            // .service( web::resource("/me").route(
            //         web::get().to(users::controllers::auth::get_me)
            // ))
    });

    // Login
    // let hashed_password = hash_password("password").expect("Error hashing password");
    let form = super::AuthData { login: String::from("login"), password: String::from("password")};
    let req = srv.post("/auth").timeout(Duration::new(15, 0));
    let response = req.send_form(&form).await.unwrap();
    // let result: CommandResult = response.json().await.expect("Could not parse json"); 
    //  Logout
    let mut req = srv
        // .delete("/auth")
        .request(http::Method::DELETE, srv.url("/auth"))
        .timeout(Duration::new(15, 0));
    req = keep_session(response, req); //Via session cookie
                        
    let response = req.send().await.unwrap();
    // let result: CommandResult = response.json().await.expect("Could not parse json"); 

    let mut req = srv.get("/auth").timeout(Duration::new(15, 0));
    req = keep_session(response, req); //Via session cookie
    let response = req.send().await.unwrap();
    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
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
