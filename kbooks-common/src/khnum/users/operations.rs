use crate::khnum::schema::{users};
use crate::khnum::wiring::{ CommandResult, DbPool };
use crate::khnum::errors::ServiceError;
use crate::khnum::users::models::{SlimUser, User};
use crate::khnum::users::repository::user_handler;

pub fn check_existence(pool: DbPool, email: &str, login: &str) -> Result<CommandResult, ServiceError> {
    let res = user_handler::fetch(pool, email, login);
    match res {
        Ok(users) => {
            if users.len() == 0 {
                return Ok(CommandResult::success());
            }
            let mut err_message = "Username already taken";
            let same_email: Vec<&SlimUser> = users.iter().filter(|user| &user.email == email).collect();
            if same_email.len() > 0 {
                err_message = "Email already taken";
            }
            Ok(CommandResult::error(String::from(err_message)) )
        }
        Err(err) => {
            println!("Error when looking unicity : {}", err);
            Err(err.into())
        }
    }
}

