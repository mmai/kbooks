use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Header, Validation};
use percent_encoding::{utf8_percent_encode, percent_decode, AsciiSet, CONTROLS};

use kbooks_common::khnum::errors::ServiceError;
use kbooks_common::khnum::users::models::SlimUser;

// cf. https://github.com/servo/rust-url/blob/master/UPGRADING.md#upgrading-from-percent-encoding-1x-to-2x
// and https://docs.rs/url/1.4.0/src/url/percent_encoding.rs.html#63
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const DEFAULT_ENCODE_SET: &AsciiSet = &QUERY_ENCODE_SET.add(b'`').add(b'?').add(b'{').add(b'}');
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &DEFAULT_ENCODE_SET.add(b'%').add(b'/');

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match std::env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}

pub fn to_url(plain: &String) -> String {
    utf8_percent_encode(plain, PATH_SEGMENT_ENCODE_SET).to_string()
}

pub fn from_url(data: &str) -> String {
    percent_decode(data.as_bytes()).decode_utf8().unwrap().to_string()
}

/* XXX for JWT tokens ...
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    email: String,
    // user login
    login: String,
}

// struct to get converted to token and back
impl Claims {
    fn with_email(email: &str, login: &str) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            email: email.to_owned(),
            login: login.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.email,
            login: claims.login,
        }
    }
}

pub fn create_token(data: &SlimUser) -> Result<String, ServiceError> {
    let claims = Claims::with_email(data.email.as_str(), data.login.as_str());
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError)
}

pub fn decode_token(token: &str) -> Result<SlimUser, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized("-".into()))?
}

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "my secret".into())
}
// end for JWT tokens
*/
