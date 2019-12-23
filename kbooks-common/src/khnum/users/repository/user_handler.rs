use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use uuid::Uuid;

use crate::khnum::wiring::DbPool;
use crate::khnum::errors::ServiceError;

use crate::khnum::schema::users::dsl;
use crate::khnum::users::models::{SlimUser, User, NewUser};

pub fn add(pool: DbPool, email: String, login: String, password: String, language: &'static str) -> Result<SlimUser, ServiceError> {
    let conn = &pool.get().unwrap();
    let user = NewUser::with_details(login, email, password, String::from(language));
    #[cfg(not(test))]
    let inserted_user: User = diesel::insert_into(dsl::users).values(&user).get_result(conn)?;
    #[cfg(test)]
    diesel::insert_into(dsl::users).values(&user).execute(conn)?;
    #[cfg(test)]
    let inserted_user: User = dsl::users.order(dsl::id.desc()).first(conn)?;

    // let expire_date = (&inserted_user).expires_at.unwrap();
    return Ok(inserted_user.into());
}

pub fn update_password(pool: DbPool, login: String, password: String) -> Result<(), ServiceError> {
    let conn = pool.get().unwrap();
    #[cfg(test)]
    diesel::update(dsl::users.filter(dsl::login.eq(login)))
        .set(dsl::password.eq(password))
        .execute(&conn)?;

    #[cfg(not(test))]
    let updated_row: Result<User, diesel::result::Error> = diesel::update(dsl::users.filter(dsl::login.eq(login)))
        .set(dsl::password.eq(password))
        .get_result(&conn);

    return Ok(());
}

pub fn email_exists(pool: DbPool, email: &String) -> Result<bool, ServiceError> {
    use crate::khnum::schema::users::dsl;
    let conn = &pool.get().unwrap();
    // let items = dsl::users.filter( dsl::email.eq(email)).load::<User>(conn)?;
    // Ok(items.into_iter().map(|item| item.into()).collect())
    diesel::dsl::select(diesel::dsl::exists(dsl::users.filter(dsl::email.eq(email))))
        .get_result(conn)
        .map_err(|err| err.into())
    // match (res){
    //     Ok(exists) => Ok(exists),
    //     diesel::result::Error => ServiceError(err)
    // }
}

pub fn fetch(pool: DbPool, email: &String, login: &String) -> Result<Vec<SlimUser>, ServiceError> {
    use crate::khnum::schema::users::dsl;
    let conn = &pool.get().unwrap();
    let items = dsl::users.filter(
        dsl::email.eq(email)
        .or( dsl::login.eq(login))
    ).load::<User>(conn)?;
    return Ok(items.into_iter().map(|item| item.into()).collect());
}
