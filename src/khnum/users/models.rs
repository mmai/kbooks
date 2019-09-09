use chrono::{Local, Duration, NaiveDateTime};
use std::convert::From;
use uuid::Uuid;

use crate::khnum::schema::{users};

#[derive(Debug, Serialize, Deserialize, Queryable)]
// XXX keep same field order as in schema.rs
pub struct User {
    pub id: i32,
    pub email: String,
    pub login: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub language: String,
}

impl User {
    pub fn testUser () -> Self {
        User {
            id: 1,
            email: "email@test.fr".to_string(),
            login: "login".to_string(),
            password: "password".to_string(),
            created_at: Local::now().naive_local(),
            language: "FR".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub login: String,
    pub email: String,
    pub password: String,
    pub language: String,
    pub created_at: NaiveDateTime,
}

impl NewUser {
    pub fn with_details(login: String, email: String, password: String, language: String) -> Self {
        NewUser {
            login,
            email,
            password,
            language,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontUser {
    pub email: String,
    pub login: String,
    pub language: String,
}
impl From<User> for FrontUser {
    fn from(user: User) -> Self {
        FrontUser { 
            login: user.login,
            email: user.email,
            language: user.language,
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SlimUser {
    pub login: String,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { 
            login: user.login,
            email: user.email,
        }
    }
}

#[test]
fn user_with_details() {
    let u = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), String::from("pass"), String::from("fr_FR"));

    assert_eq!(u.login , String::from("login"));
}

#[test]
fn slim_from_user() {
    let u = User {
        id: 1,
        login: "login".into(),
        email: "email@toto.fr".into(),
        password: "pass".into(),
        created_at: Local::now().naive_local(),
        language: "en_US".into(),
    };
    let s:SlimUser = u.into();
    assert_eq!(SlimUser { login: "login".into(), email: "email@toto.fr".into() }, s);
}

#[test]
fn front_from_user() {
    let u = User {
        id: 1,
        login: "login".into(),
        email: "email@toto.fr".into(),
        password: "pass".into(),
        created_at: Local::now().naive_local(),
        language: "en_US".into(),
    };
    let s:FrontUser = u.into();
    assert_eq!(FrontUser { login: "login".into(), email: "email@toto.fr".into(), language: "en_US".into() }, s);
}
