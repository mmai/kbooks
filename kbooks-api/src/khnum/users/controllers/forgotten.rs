use actix_session::{CookieSession, Session};
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

use kbooks_common::khnum::wiring::{DbPool, Config};
use kbooks_common::khnum::errors::ServiceError;

use kbooks_common::khnum::users::repository::user_handler;
use kbooks_common::khnum::users::models::{SlimUser, User};
use crate::khnum::users::utils::{hash_password, to_url, from_url};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    success: bool,
    error: Option<String>
}

// ---------------- Request Action------------

// UserData is used to extract data from a post request by the client
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestForm {
    email: String
}

pub async fn request(
    form_data: web::Form<RequestForm>,
    config: web::Data<Config>
) -> Result<HttpResponse, ServiceError> {
    let form_data = form_data.into_inner();

    let email_exists = user_handler::email_exists(config.pool.clone(), &form_data.email).expect("error when checking email");
    if !email_exists {
        let cde_res = CommandResult {success: false, error: Some(String::from("Email does not exists"))};
        Ok(HttpResponse::Ok().json(cde_res))
    } else {
        let expires_at = Local::now().naive_local() + Duration::hours(24);
        let res = send_confirmation(form_data.email, expires_at);
        Ok(HttpResponse::Ok().json(res))
    }
}

// ---------------- Validate link action ------------
pub async fn check( 
    session: Session,
    data: web::Path<(String, String, String)>, 
    ) 
    // -> impl Future<Item = HttpResponse, Error = Error> {
    -> Result<HttpResponse, ServiceError> {

    //Verify link
    let hashlink = from_url(&data.0);
    let email = from_url(&data.1);
    let expires_at: i64 = data.2.clone().parse().unwrap();
    let validate_params = format!("{}{}", email, expires_at);
    let local_link = make_confirmation_data(&validate_params);
    let validate_result = verify(local_link, &hashlink[..])
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
            if session.set("email", email ).is_ok() {
                // println!("email in session : {:#?}", session.get::<String>("email").expect("err while getting email from session"));
                CommandResult { success: true, error: None }
            } else {
                CommandResult { success: false, error: Some(String::from("Could not save email in session")) }
            }
        });
    // println!("{:#?}", validate_result);
    match validate_result {
        Err(res) => Ok(HttpResponse::Ok().json(res)),
        Ok(res) => Ok(HttpResponse::Ok().json(res))
    }
}

// -------- 
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordForm {
    password: String,
}

pub async fn change_password(
    session: Session,
    form_data: web::Form<PasswordForm>,
    config: web::Data<Config>,
// ) -> impl Future<Item = HttpResponse, Error = ServiceError> {
) -> Result<HttpResponse, Error> {
    let form_data = form_data.into_inner();

    let res = {
        let opt = session.get::<String>("email").expect("could not get session email");
        let email = opt.unwrap();
        let email_exists = user_handler::email_exists(config.pool.clone(), &email).expect("error when checking email");
        if !email_exists {
            let cde_res = CommandResult {success: false, error: Some(String::from("Email does not exists"))};
            Ok(HttpResponse::Ok().json(cde_res))
        } else {
            let _user = user_handler::update_password(config.pool.clone(), email, form_data.password).expect("error when updating password");
            Ok( HttpResponse::Ok().json(CommandResult {success: true, error: None}))
        }
    };
    res
}

fn make_confirmation_data(msg: &str) -> String {
    let key = dotenv::var("SECRET_KEY").unwrap();
    format!("{}{}", msg, key)
}

fn send_confirmation(email: String, expires_at: NaiveDateTime) -> CommandResult {
    let validate_params = format!("{}{}", email, expires_at.timestamp());
    // println!("{}{}", email, expires_at.timestamp());

    let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("SENDING_EMAIL_ADDRESS must be set");
    let base_url = dotenv::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let recipient = &email[..];
    let link = make_confirmation_data(&validate_params);
    let confirmation_hash = hash_password(&link)
        .map(|hash| to_url(&hash))
        .expect("Error hashing link");
    let url = format!("{}/user/changepassword/{}/{}/{}", base_url, confirmation_hash, to_url(&email), expires_at.timestamp());
    let email_body = format!(
        "Please click on the link below to change your password. <br/>
         <a href=\"{url}\">{url}</a> <br>
         link expires on <strong>{date}</strong>",
         url = url,
         date = expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );
    // println!("{}", email_body);
    // println!("{}", recipient);

    let email = Email::builder()
        .from((sending_email, "khnum"))
        .to(recipient)
        .subject("Password reset")
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

    // let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    let sendmail = dotenv::var("SENDMAIL").unwrap_or_else(|_| "/usr/sbin/sendmail".to_string()); 
    let mut mailer = SendmailTransport::new_with_command(sendmail);

    // We don't send the mail in test environment
    #[cfg(test)]
    return CommandResult {success: true, error: None};

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
