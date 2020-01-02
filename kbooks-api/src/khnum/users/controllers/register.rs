use actix_session::{CookieSession, Session};
use actix_http::cookie::Cookie;
use actix_web::{ test, web, Error, error, HttpResponse, ResponseError, http};
use bcrypt::verify;
use chrono::{Duration, Local, NaiveDateTime};
use futures::future::{Future, err};

use url::form_urlencoded;

use lettre_email::Email;
use lettre::{SmtpClient, Transport};
use lettre::file::FileTransport;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::sendmail::SendmailTransport;

use kbooks_common::khnum::wiring::{DbPool, Config, make_front_url};
use kbooks_common::khnum::errors::ServiceError;

use kbooks_common::khnum::users::repository::user_handler;
use kbooks_common::khnum::users::models::{SlimUser, User};
use crate::khnum::users::utils::{hash_password, to_url, from_url};

use actix_i18n::I18n;
use gettext::Catalog;
use gettext_macros::i18n;


use std::fs::File;
use std::io::prelude::*;

fn error_log(mess: &str) {
    let mut logfile = File::create("/tmp/kbooks-api_error.log").unwrap();
    logfile.write(mess.as_bytes()).unwrap();
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    success: bool,
    error: Option<String>
}

// ---------------- Request Action------------

// UserData is used to extract data from a post request by the client
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestForm {
    email: String,
    username: String,
    password: String
}

pub async fn request(
    form_data: web::Form<RequestForm>,
    config: web::Data<Config>,
    i18n: I18n
) -> Result<HttpResponse, ServiceError> {
    let form_data = form_data.into_inner();
    let res = check_existence(config.pool.clone(), &form_data.email, &form_data.username);
    match res {
        Ok(cde_res) => {
            if !cde_res.success {
                Ok(HttpResponse::Ok().json(cde_res))
            } else {
                let hashed_password = hash_password(&form_data.password).expect("Error hashing password");
                let expires_at = Local::now().naive_local() + Duration::hours(24);
                // panic!(" avant send_confirmation");
                let res = send_confirmation(&i18n.catalog, form_data.username, hashed_password, form_data.email, expires_at);
                Ok(HttpResponse::Ok().json(res))
            }
        }
        Err(err) => {
            error_log(&format!("check existence {:?}", err));
           Err(err)
        }
    }
}

// ---------------- Validate link action and finish registration ------------
pub async fn register( 
    config: web::Data<Config>,
    i18n: I18n,
    data: web::Path<(String, String, String, String, String)>, 
    ) 
    -> Result<HttpResponse, ServiceError> {

    //Verify link
    let hashlink = from_url(&data.0);
    let username = from_url(&data.1);
    let hpasswd = from_url(&data.2);
    let email = from_url(&data.3);
    let expires_at: i64 = data.4.clone().parse().unwrap();
    let validate_params = format!("{}{}{}{}", username, hpasswd, email, expires_at);
    let local_link = make_confirmation_data(&validate_params);

    let validate_result = verify(local_link.clone(), &hashlink[..])
        .map_err(|_err| 
            CommandResult { success: false, error: Some(String::from("Invalid hash link")) }
        )
        .map(|is_valid| {
            if !is_valid {
                return CommandResult { success: false, error: Some(String::from("Incorrect link")) };
            }
            let now = Local::now().naive_local().timestamp();
            if expires_at < now {
                return CommandResult { success: false, error: Some(String::from("Link validity expired")) };
            }
            
            let check_existence_res = check_existence(config.pool.clone(), &email, &username).expect("error when checking existence");
            if !check_existence_res.success {
                check_existence_res
            } else {
                let _user = user_handler::add(config.pool.clone(), email, username, hpasswd, &i18n.lang).expect("error when inserting new user");
                CommandResult {success: true, error: None}
            }

        });

    match validate_result {
        Err(res) => Ok(HttpResponse::Ok().json(res)),
        Ok(res) => {
            if res.success {
                // let _ = session.set("flashmessage", "Thank your for registering. You can now log in");
                // let cookie: Cookie = Cookie::build("action", "registerOk")
                //     .domain("localhost:8080")
                //     .path("/")
                //     .secure(true)
                //     .http_only(true)
                //     .max_age(84600)
                //     .finish();
                Ok(
                            HttpResponse::Found()
                            .header(http::header::LOCATION, make_front_url(&config.front_url, "/?action=registerOk") )
                            // .cookie(cookie)
                            .finish()
                            .into_body()
                )
            } else {
                Ok(HttpResponse::Ok().json(res))
            }
        }
    }
}

// -------- 
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateForm {
    username: String,
    password: String,
}

fn check_existence(pool: DbPool, email: &String, login: &String) -> Result<CommandResult, ServiceError> {
    let res = user_handler::fetch(pool, email, login);
    match res {
        Ok(users) => {
            if users.len() == 0 {
                return Ok(CommandResult {success: true, error: None});
            }
            let mut err_message = "Username already taken";
            let same_email: Vec<&SlimUser> = users.iter().filter(|user| &user.email == email).collect();
            if same_email.len() > 0 {
                err_message = "Email already taken";
            }
            Ok(CommandResult {success: false, error: Some(String::from(err_message))})
        }
        Err(err) => {
            println!("Error when looking unicity : {}", err);
            Err(err.into())
        }
    }
}

fn make_confirmation_data(msg: &str) -> String {
    let key = dotenv::var("SECRET_KEY").unwrap();
    format!("{}{}", msg, key)
}

fn make_register_link(base_url: &String, username: &String, hpassword: &String, email: &String, expires_at: i64) -> String {
    let validate_params = format!("{}{}{}{}", username, hpassword, email, expires_at);
    let link = make_confirmation_data(&validate_params);
    let confirmation_hash = hash_password(&link)
        .expect("Error hashing link");
    let url = format!("{}/register/register/{}/{}/{}/{}/{}", base_url, to_url(&confirmation_hash), to_url(&username), to_url(&hpassword), to_url(&email), expires_at);
    url
}

fn send_confirmation(catalog: &Catalog, username: String, hpassword: String, email: String, expires_at: NaiveDateTime) -> CommandResult {
    println!("{}{}", email, expires_at.timestamp());

    let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("SENDING_EMAIL_ADDRESS must be set");
    let base_url = dotenv::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let url = make_register_link(&base_url, &username, &hpassword, &email, expires_at.timestamp());
    let recipient = &email[..];
    let email_body = format!(
        "{msg_click}. <br/>
         <a href=\"{url}\">{url}</a> <br>
        {msg_expire}  <strong>{date}</strong>",
         msg_click = i18n!(catalog, "Please click on the link below to complete registration"), 
         msg_expire = i18n!(catalog, "your Invitation expires on"),
         url = url,
         date = expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );
    // panic!("{}", email_body);
    // println!("{}", recipient);

    let email = Email::builder()
        .from((sending_email, "khnum"))
        .to(recipient)
        .subject("You have been invited to join khnum")
        .html(email_body)
        .build();
    assert!(email.is_ok());

    // let smtp_login = dotenv::var("SMTP_LOGIN").unwrap_or_else(|_| "user".to_string());
    // let smtp_pass = dotenv::var("SMTP_PASSWORD").unwrap_or_else(|_| "password".to_string());
    // let smtp_server = dotenv::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.localhost".to_string()); 
    // let creds = Credentials::new( smtp_login, smtp_pass );
    // let mut mailer = SmtpClient::new_simple(&smtp_server)
    //     .unwrap()
    //     .credentials(creds)
    //     .transport();

    // We don't send the mail in test environment
    #[cfg(test)]
    return CommandResult {success: true, error: None};

    // let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    let sendmail = dotenv::var("SENDMAIL").unwrap_or_else(|_| "/usr/sbin/sendmail".to_string()); 
    let mut mailer = SendmailTransport::new_with_command(sendmail);

    let result = mailer.send(email.unwrap().into());
    match result {
        Ok(_res) => CommandResult {success: true, error: None} ,
        Err(error) => {
            // println!("error \n {:#?}", error);
            CommandResult {success: false, error: Some(format!("Error sending mail. {:#?}", error))}
        }
    }
}

#[cfg(test)]
mod tests;
// #[path = "./register_test.rs"] // avoid creating a /register folder
// mod register_test;
