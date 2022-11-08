use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::{
    components::{collection::Collection, mappings::Mapping, user::User},
    utils::{
        collection::auto_fetch_all_collections, config::get_config_value,
        data::auto_fetch_all_data, mapping::auto_fetch_all_mappings, user::auto_fetch_all_users,
    },
};

pub struct Token(pub String);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    payload: String,
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

pub fn create_jwt(mappings: &Vec<Mapping>, payload: String) -> Result<String, String> {
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
        payload: payload.to_owned(),
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

pub async fn verify_jwt(payload: String, token: String) -> Result<String, (usize, String)> {
    let mappings = auto_fetch_all_mappings();
    let secret = get_config_value(&mappings, "TOKEN_KEY", "secret");
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return Err((500, String::from("Error: Failed fetching users")));
        }
    };

    if !User::exist(&users, &payload) {
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

    if decoded.claims.payload != payload {
        return Err((403, String::from("Error: Incorrect UID")));
    }

    Ok(String::from("Successfully Authenticated!"))
}

pub async fn verify_jwt_x(
    payload: String,
    token: String,
    project_id: &str,
    ref_col: &str,
    field: &str,
) -> Result<String, (usize, String)> {
    let mappings = auto_fetch_all_mappings();
    let secret = get_config_value(&mappings, "TOKEN_KEY", "secret");

    if ref_col.trim().len() < 1 || field.trim().len() < 1 {
        return Err((500, String::from("Error: Invalid field or ref_col")));
    }

    let all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return Err((500, String::from("Error: Failed fetching collections")));
        }
    };

    let collection = match Collection::get(&all_collections, project_id, ref_col) {
        Ok(col) => col,
        Err(_) => {
            return Err((
                404,
                String::from("Error: No Collection with this collection_id found"),
            ));
        }
    };

    let mut exists = false;
    for structure in collection.structures {
        if structure.id == field {
            exists = true;
            break;
        }
    }

    if !exists {
        return Err((
            404,
            format!("Error: No Structure corresponding to {} found", field),
        ));
    }

    let all_data = match auto_fetch_all_data(&mappings, &project_id, ref_col) {
        Ok(d) => d,
        _ => {
            return Err((500, String::from("Error: Failed fetching data")));
        }
    };

    let mut found = false;
    for d in all_data {
        for pair in d.pairs {
            if pair.structure_id == field && pair.value == payload {
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    if !found {
        return Err((404, format!("Error: Data with this {} not found", field)));
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

    if decoded.claims.payload != payload {
        return Err((403, format!("Error: Incorrect {}", field)));
    }

    Ok(String::from("Successfully Authenticated!"))
}
