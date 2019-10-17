use actix_session::{Session};
use actix_web::{ test, web, Error, error, HttpResponse, ResponseError, http};
use chrono::{Duration, Local, NaiveDateTime, Utc };
use futures::future::{Future, result, err};

//For tests
use dotenv::dotenv;
use actix_web::{ App};
// use actix_web::{web, test, http, App};
use actix_http::HttpService;
use actix_http_test::TestServer;
use actix_i18n::Translations;
use gettext_macros::include_i18n;

use crate::khnum::wiring::{DbPool, Config, make_front_url};
use crate::khnum::errors::ServiceError;
use crate::khnum::users;

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
    author: String,
    isbn: String,
    publicationdate: String,
    language_main: String,
    language_secondary: Option<String>,
    language_original: String,
}

//TODO
fn get_or_create_author_code(author: &String) -> String {
    "ROUBAUD".to_string()
}

pub fn create(
    session: Session,
    book_form: web::Form<NewBookForm>,
    config: web::Data<Config>,
    i18n: I18n
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    //TODO : bad input data
    let book_form = book_form.into_inner();

    #[cfg(test)]
    let opt = Some(users::models::User::testUser());

    #[cfg(not(test))]
    let opt = session.get::<users::models::User>("user").expect("could not get session user");

    match opt {
        None => result(Err(ServiceError::Unauthorized("User not connected".to_string()))),
        Some(user) => {
            let author_code = get_or_create_author_code(&book_form.author);
            let book = NewBook {
                user_id: user.id, 
                librarything_id: None,
                title: book_form.title,
                author_lf: book_form.author,
                author_code,
                isbn: book_form.isbn,
                publicationdate: book_form.publicationdate,
                rating: None,
                language_main: book_form.language_main,
                language_secondary: book_form.language_secondary,
                language_original: book_form.language_original,
                review: None,
                cover: "".to_string(),
                created_at: Utc::now().naive_utc(),
                dateacquired_stamp: None,
                started_stamp: None,
                finished_stamp: None
            };

            //TODO : db error
            let _book = book_handler::add(config.pool.clone(), book).expect("error when inserting new book");
            let res = CommandResult {success: true, error: None};
            result(Ok(HttpResponse::Ok().json(res)))
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooksListCommandResult {
    success: bool,
    books: Vec<Book>,
    error: Option<String>
}

pub fn list(
    session: Session,
    config: web::Data<Config>,
    i18n: I18n
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    //TODO : bad input data

    #[cfg(test)]
    let opt = Some(users::models::User::testUser());

    #[cfg(not(test))]
    let opt = session.get::<users::models::User>("user").expect("could not get session user");

    match opt {
        None => result(Err(ServiceError::Unauthorized("User not connected".to_string()))),
        Some(user) => {
            //TODO : db error
            let books = book_handler::list(config.pool.clone()).expect("error getting books list");
            let res = BooksListCommandResult {success: true, books, error: None};
            result(Ok(HttpResponse::Ok().json(res)))
        },
    }
}

// #[cfg(test)]
// mod tests;
//// #[path = "./book_test.rs"] // avoid creating a /register folder
//// mod book_test;


pub fn managed_state() -> Translations {
    include_i18n!()
}

#[test]
fn test_create() {
    dotenv().ok();
    let mut srv = TestServer::new( || {
        let pool = crate::khnum::wiring::test_conn_init();
        let conn = &pool.get().unwrap();
        HttpService::new(
            App::new()
            .data(managed_state())
            .data(Config {pool: pool.clone(), front_url: String::from("http://dummy")}).service(
                web::scope("/book")
                    .service( web::resource("/create").route(
                        web::post().to_async(create)
                    )
                )
            )
        )
    });

    let form = NewBookForm { 
        title: "Le grand incendie de Londres".to_string(),
        author: "Roubaud, Jacques".to_string(),
        isbn: "2020104725".to_string(),
        publicationdate: "1989-01".to_string(),
        language_main: "FR".to_string(),
        language_secondary: None,
        language_original: "FR".to_string(),
    };

    let req = srv.post("/book/create")
        .timeout(std::time::Duration::new(15, 0));
        // .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

    let mut response = srv.block_on(req.send_form(&form)).unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(result.success);
}

use diesel::prelude::*;
use crate::schema::books::dsl;
#[test]
fn test_list() {
    dotenv().ok();
    let mut srv = TestServer::new( || {
        let pool = crate::khnum::wiring::test_conn_init();
        let conn = &pool.get().unwrap();
        let book = NewBook {
            user_id: 1,
            librarything_id: None,
            title: "a title".to_string(),
            author_lf: "Authorlf".to_string(),
            author_code: "AUT".to_string(),
            isbn: "1234564654654654645".to_string(),
            publicationdate: "2019-03-02".to_string(),
            language_original: "FR".to_string(),
            language_main: "FR".to_string(),
            language_secondary: None,
            review: None,
            rating: None,
            cover: "".to_string(),
            created_at: Utc::now().naive_utc(),
            dateacquired_stamp: None,
            started_stamp: None,
            finished_stamp: None,
        };
        diesel::insert_into(dsl::books).values(&book)
            .execute(conn).expect("Error populating test database");
        HttpService::new(
            App::new()
            .data(managed_state())
            .data(Config {pool: pool.clone(), front_url: String::from("http://dummy")})
            .service( web::resource("/book")
                      .route( web::get().to_async(list))
            )
        )
    });

    let req = srv.get("/book")
        .timeout(std::time::Duration::new(15, 0));
        // .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

    let mut response = srv.block_on(req.send()).unwrap();
    assert!(response.status().is_success());
    let result: BooksListCommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(result.books.len() == 1);
}
