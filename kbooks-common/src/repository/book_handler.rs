use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use diesel::result::Error as DBError;
use uuid::Uuid;

use crate::khnum::wiring::DbPool;

use crate::schema::books::dsl;
use crate::models::{Book, NewBook};

pub fn add(pool: DbPool, book: NewBook) -> Result<(), DBError> {
    let conn = &pool.get().unwrap();
    #[cfg(not(feature = "test"))]
    let inserted_book: Book = diesel::insert_into(dsl::books).values(&book).get_result(conn)?;
    #[cfg(feature = "test")]
    diesel::insert_into(dsl::books).values(&book).execute(conn)?;
    #[cfg(feature = "test")]
    let inserted_book: Book = dsl::books.order(dsl::id.desc()).first(conn)?;

    // let expire_date = (&inserted_user).expires_at.unwrap();
    return Ok(());
}

pub fn list(pool: DbPool) -> Result<Vec<Book>, DBError> {
    let conn = &pool.get().unwrap();
    let items = dsl::books.load::<Book>(conn)?;
    return Ok(items.into_iter().map(|item| item.into()).collect());
}

// pub fn fetch(pool: DbPool, email: &String, login: &String) -> Result<Vec<SlimUser>, DBError> {
//     use crate::khnum::schema::users::dsl;
//     let conn = &pool.get().unwrap();
//     let items = dsl::users.filter(
//         dsl::email.eq(email)
//         .or( dsl::login.eq(login))
//     ).load::<User>(conn)?;
//     return Ok(items.into_iter().map(|item| item.into()).collect());
// }
