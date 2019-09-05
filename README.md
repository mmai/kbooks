# Kbooks

A books library application built on top of Khnum (WIP)

## Installation

Init postgresql database (with docker)

```sh
cd myproject
make initdb
make migrate
```

Fetch frontend dependencies

```sh
cd front
yarn
```

## Start application in development mode

Start backend server

```sh
make run
```
Start frontend development server

```sh
make frontrun
```
