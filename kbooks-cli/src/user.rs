use clap::{App, Arg, ArgMatches, SubCommand};

use kbooks_common::khnum::wiring;
use kbooks_common::khnum::users::repository::user_handler;
use kbooks_common::khnum::users::operations::check_existence;

pub const name: &str = "user";

pub fn add_command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b>  {
    app.subcommand(
    SubCommand::with_name(name)
        .about("user administration")
        .subcommand(
            SubCommand::with_name("add").about("add a new user")
            .arg(
                Arg::with_name("NAME")
                .help("name of the user to add")
                .required(true)
            )
        ))
}

pub fn actions(matches: &ArgMatches) {
    // Add user
    if let Some(matches) = matches.subcommand_matches("add") {
        // dotenv().ok();
        // let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = wiring::db_init(String::from("postgres://dbuser:password@localhost:5432/kbooks"));

        let username = matches.value_of("NAME").unwrap_or("admin");
        println!("Name of user: {:?} ", username);

       let email = "test@test.fr";
       let check_existence_res = check_existence(pool.clone(), &email, username).expect("error when checking existence");
       if !check_existence_res.is_success() {
           println!("User already exists");
           // check_existence_res
       } else {
           let _user = user_handler::add(pool.clone(), email, username, "xxxx", "fr").expect("error when inserting new user");
           println!("User successfully added");
           // CommandResult {success: true, error: None}
       }
    }
}

