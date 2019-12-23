# Kbooks

A books library application built on top of Khnum (WIP)


## Dependencies

You can create a developpement environment on nix enabled system by executing `nix-shell`

This installs: 

* sqlite (for tests)
* postgresql
* docker-compose
* gettext
* openssl, pkgconfig (needed for installing various cargo packages)

It also set the database connexion parameters

## Installation

Init postgresql database (with docker)

```sh
cd myproject
make initdb
make migrate
```


### Troubleshooting 

* `error libmariadb.so.x not found` : reinstall _diesel_cli_ with `cargo install diesel_cli --no-default-features --features postgres,sqlite`

## Create documentation

`make doc`

## Tests

`make test`

tests with coverage : `make coverage`

## Start application in development mode

Start backend server

```sh
make run
```
