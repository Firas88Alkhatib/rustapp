use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error as ActixError};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use argon2::{Config, Error as HashingError};
use rand::Rng;

const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET_KEY: &str = "SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub permissions: Vec<String>,
    exp: i64,
}

// Role - is permission with prefix "ROLE_".
impl Claims {
    pub fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

pub fn create_jwt(claims: Claims) -> Result<String, ActixError> {
    let encoding_key = EncodingKey::from_secret(SECRET_KEY.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key).map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<Claims, ActixError> {
    let decoding_key = DecodingKey::from_secret(SECRET_KEY.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let result = decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}

pub fn hash_password(password: String) -> Result<String, HashingError> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    let hashed = argon2::hash_encoded(password.as_bytes(), &salt, &config);

    return hashed;
}
pub fn verify_password(hashed_password: String, password: String) -> Result<bool, HashingError> {
    argon2::verify_encoded(&hashed_password, password.as_bytes())
}
