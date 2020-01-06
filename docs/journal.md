# Dev journal

## Tools

* Paperclip (OpenAPI code generator) `cargo +nightly-2019-11-27 install paperclip --features cli`

## Init

```
diesel migration run --migration-dir migrations/khnum/postgres
```

## New entity

1. Database migration

```
diesel migration generate --migration-dir migrations/postgres books
vim ./migrations/postgres/2019-09-02-161617_books/up.sql
vim ./migrations/postgres/2019-09-02-161617_books/down.sql
diesel migration run --migration-dir migrations/postgres
```

2. Model

```
vim ./src/models.rs
```

=> struct Book ; struct NewBook 

