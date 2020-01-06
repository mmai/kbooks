# Howto

## Modify database schema

* `diesel migration generate --migration-dir migrations/postgres user_language`
* `find migrations/postgres -name *user_language| xargs -I '{}' cp -R '{}' migrations/sqlite`
* edit generated .sql files
* `diesel migration run --migration-dir migrations/postgres`
* _src/schema.rs_ should be updated

## Update Rust packages

Make sure you have the latest version of the compiler : `rustup update`

Update packages : `cargo update` (or `cargo upgrade` to update _Cargo.toml_ as well )
