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

## Trying to build nix package

with `buildRustPackage` in _derivation.nix_ (mozilla for nightly) : 

```
nix build
```

=> 
```
warning: dumping very large path (> 256 MiB); this may run out of memory
error: Nix daemon out of memory
```

### Naersk to help build nix package

```
niv init
niv add nmattia/naersk
```

=> error 

### Crate2nix

Installation:
```
nix-env -i -f https://github.com/kolloch/crate2nix/tarball/0.6.1
```
Usage :
```
crate2nix generate -o Cargo.nix

nix build -f Cargo.nix workspaceMembers.kbooks-api.build
```
=> error
