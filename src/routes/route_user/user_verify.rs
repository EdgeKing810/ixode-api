use rocket::get;
use rocket::serde::json::{json, Value};

use crate::middlewares::token::{verify_jwt, Token};

#[get("/verify?<uid>")]
pub async fn main(token: Token, uid: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": "401", "message": "Error: No uid provided"}),
    };

    return match verify_jwt(passed_uid.clone(), token.0).await {
        Ok(msg) => json!({"status": 200, "message": msg}),
        Err(info) => json!({"status": info.0, "message": info.1}),
    };
}
