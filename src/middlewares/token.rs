use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::{
    components::{mappings::Mapping, user::User},
    utils::{auto_fetch_all_mappings, auto_fetch_all_users, get_config_value},
};

pub struct Token(pub String);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    uid: String,
    exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req.headers().get_one("authorization");

        match token {
            Some(token) => Outcome::Success(Token(token.to_string())),
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn create_jwt(mappings: &Vec<Mapping>, uid: String) -> Result<String, String> {
    let expire = match get_config_value(mappings, "JWT_EXPIRE", "900").parse::<i64>() {
        Ok(val) => val,
        _ => 900,
    };
    let secret = get_config_value(mappings, "TOKEN_KEY", "secret");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expire))
        .expect("Valid Timestamp")
        .timestamp();

    let claims = Claims {
        uid: uid.to_owned(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    return match encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(String::from("Error: Failed to create JWT")),
    };
}

pub async fn verify_jwt(uid: String, token: String) -> Result<String, (usize, String)> {
    let mappings = auto_fetch_all_mappings();
    let secret = get_config_value(&mappings, "TOKEN_KEY", "secret");
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return Err((500, String::from("Error: Failed fetching users")));
        }
    };

    if !User::exist(&users, &uid) {
        return Err((404, String::from("Error: Account with this uid not found")));
    }

    let mut proper_token = "";
    for v in token.split(" ") {
        proper_token = v;
    }

    let decoded = match decode::<Claims>(
        &proper_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(dec) => dec,
        Err(e) => {
            return Err((
                500,
                format!("{} ({})", String::from("Error: Failed decoding JWT"), e),
            ));
        }
    };

    if decoded.claims.uid != uid {
        return Err((403, String::from("Error: Incorrect UID")));
    }

    Ok(String::from("Successfully Authenticated!"))
}
