use diesel::prelude::{Connection, PgConnection, SqliteConnection};
use diesel::r2d2;

// #[cfg(not(any(feature = "sqlite", feature = "postgres")))]
// compile_error!("Either feature \"sqlite\" or \"postgres\" must be enabled for this crate.");
// #[cfg(all(feature = "sqlite", feature = "postgres"))]
// compile_error!("Features \"sqlite\" and \"postgres\" should not be enabled at the same time.");

// #[cfg(feature = "diesel/sqlite")]
// #[cfg(all(feature = "diesel/sqlite", not(feature = "diesel/postgres")))]
// #[cfg(test)]
#[cfg(feature = "test")]
pub type MyConnection = SqliteConnection;

// #[cfg(not(feature = "diesel/sqlite"))]
// #[cfg(all(not(feature = "diesel/sqlite"), feature = "diesel/postgres"))]
// #[cfg(not(test))]
#[cfg(not(feature = "test"))]
pub type MyConnection = PgConnection;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<MyConnection>>;

pub struct Config {
    pub pool: DbPool,
    pub front_url: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    success: bool,
    error: Option<String>
}

impl CommandResult {
    pub fn get_error(self) -> Option<String> {
        self.error
    }

    pub fn is_success(self) -> bool {
        self.success
    }

    pub fn success() -> CommandResult {
        CommandResult {
            success: true,
            error: None
        }
    }

    pub fn error(message: String) -> CommandResult {
        CommandResult {
            success: false,
            error: Some(message)
        }
    }
}


pub fn make_front_url (root_url: &String, url: &str) -> String {
    format!("{}/#{}", root_url, url)
}

#[cfg_attr(tarpaulin, skip)]
pub fn db_init(db_url: String) -> DbPool {
    let manager = r2d2::ConnectionManager::<MyConnection>::new(db_url);
    r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.")
}

// ================== Test database initialization
// #[cfg(feature = "diesel/sqlite")]
// #[cfg(test)]
#[cfg(feature = "test")]
embed_migrations!("../migrations/sqlite");

// #[cfg(feature = "diesel/sqlite")]
#[cfg(feature = "test")]
// #[cfg(test)]
pub fn test_conn_init() -> DbPool {
    let manager = r2d2::ConnectionManager::<MyConnection>::new(":memory:");
    let pool = r2d2::Pool::builder().max_size(2).build(manager).expect("Failed to create pool.");
    let conn = &pool.get().unwrap();
    let _res = embedded_migrations::run(conn);
    pool
}

