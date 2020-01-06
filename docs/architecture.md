# Architecture

## Configuration

Sub projects :

kbooks-common : core application, with database access
kbooks-cli: command line interface
kbooks-api
  web server with actix

Configuration values are set in _.env_ file
They are red by the _kbooks-common_ package which is used by _kbooks-api_ and _kbooks-cli_

## Errors

* cf. https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/

## Domain driven design / CQRS

https://github.com/KodrAus/rust-web-app

## Users

## Registration

* initially based on https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/
* but without storing invitations : https://neosmart.net/blog/2015/using-hmac-signatures-to-avoid-database-writes/

* Hashing is done via bcrypt

Form (email) -> post to
/register/request 
  -> check email not taken
  -> send confirmation link 

/register/{hashlink}/{email}/{expires_at} 
  -> check link valid 
  -> init session with email

Form (username, password) (with session cookie) -> post to
/register/validate
  -> get session email
  -> check email not taken
  -> check username not taken
  -> hash password
  -> create user

## Forgotten  password

Form (email) -> post to
/user/forgotten 
  -> check email exists
  -> send temporary link 

/user/forgotten/{hashlink}
  -> check link valid 
  -> init session

Form (password) (with session cookie) -> post to
/user/changepassword
  -> get session user
  -> hash password
  -> update user

## Login

Form (login, password) -> post to
/login
  -> check user exist for login + password
  -> init session

protected pages :
  -> get session cookie
  -> check user exists
  -> check user rights

## Logout

/logout
 -> invalidate session cookie
