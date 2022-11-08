use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::user::User;
use crate::middlewares::token::{create_jwt, verify_jwt, Token};
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::auto_fetch_all_users;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UIDInput {
    uid: String,
}

#[post("/login/jwt", format = "json", data = "<data>")]
pub async fn main(data: Json<UIDInput>, token: Token) -> Value {
    let uid = &data.uid;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let user = match User::get(&users, uid) {
        Ok(user) => user,
        Err(e) => {
            return json!({"status": e.0, "message": e.1});
        }
    };

    let jwt = match create_jwt(&mappings, uid.clone()) {
        Ok(token) => token,
        Err(e) => return json!({"status": 500, "message": e}),
    };

    json!({"status": 200, "message": "Login Successful!", "user": user, "uid": user.id, "jwt": jwt})
}
