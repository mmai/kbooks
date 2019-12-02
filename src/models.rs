use diesel::{self,RunQueryDsl,QueryDsl,ExpressionMethods};
use actix_web::{error};
use actix::{Message,Handler};
use chrono::{Utc, NaiveDateTime};
use crate::schema::books;
use crate::khnum::errors::ServiceError;

use paperclip::actix::api_v2_schema;

#[api_v2_schema]
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Book {
    pub id: i32,
    pub user_id: i32,
    pub librarything_id: Option<String>,
    pub title: String,
    pub author_lf: String,
    pub author_code: String,
    pub isbn: String,
    pub publicationdate: String,
    pub rating: Option<i32>,
    pub language_main: String,
    pub language_secondary: Option<String>,
    pub language_original: String,
    pub review: Option<String>,
    pub cover: String,
    // pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub dateacquired_stamp: Option<NaiveDateTime>,
    pub started_stamp: Option<NaiveDateTime>,
    pub finished_stamp: Option<NaiveDateTime>
}

impl Book {
    pub fn new() -> Book {
        Book {
            id: 0,
            user_id: 0,
            librarything_id: None,
            title: "".to_string(),
            author_lf: "".to_string(),
            author_code: "".to_string(),
            isbn: "".to_string(),
            publicationdate: "".to_string(),
            rating: None,
            language_main: "".to_string(),
            language_secondary: None,
            language_original: "".to_string(),
            review: None,
            cover: "".to_string(),
            created_at: Utc::now().naive_utc(),
            dateacquired_stamp: Some(Utc::now().naive_utc()),
            started_stamp: Some(Utc::now().naive_utc()),
            finished_stamp: Some(Utc::now().naive_utc()),
        }
    }
}

// ---------------- Store insertion -------------

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="books"]
pub struct NewBook {
    pub user_id: i32,
    pub librarything_id: Option<String>,
    pub title: String,
    pub author_lf: String,
    pub author_code: String,
    pub isbn: String,
    pub publicationdate: String,
    pub rating: Option<i32>,
    pub language_main: String,
    pub language_secondary: Option<String>,
    pub language_original: String,
    pub review: Option<String>,
    pub cover: String,
    // pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub dateacquired_stamp: Option<NaiveDateTime>,
    pub started_stamp: Option<NaiveDateTime>,
    pub finished_stamp: Option<NaiveDateTime>
}
