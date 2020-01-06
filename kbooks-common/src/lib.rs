//for table!
#[macro_use] extern crate diesel;
//for Serialize, Deserialize
#[macro_use] extern crate serde_derive;
//For tests
#[macro_use] extern crate diesel_migrations;

pub mod khnum;

pub mod schema;
pub mod models;
pub mod repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
