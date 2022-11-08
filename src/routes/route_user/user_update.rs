use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::user::User;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::{auto_fetch_all_users, auto_save_all_users};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Change {
    FIRSTNAME,
    LASTNAME,
    USERNAME,
    EMAIL,
    PASSWORD,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangeInput {
    uid: String,
    change: Change,
    data: String,
}

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<ChangeInput>, token: Token) -> Value {
    let uid = &data.uid;
    let change = &data.change;
    let data = &data.data;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    match match change.clone() {
        Change::FIRSTNAME => User::update_first_name(&mut users, uid, data),
        Change::LASTNAME => User::update_last_name(&mut users, uid, data),
        Change::USERNAME => User::update_username(&mut users, uid, data),
        Change::EMAIL => User::update_email(&mut users, uid, data),
        Change::PASSWORD => User::update_password(&mut users, uid, data),
    } {
        Err(e) => {
            return json!({"status": e.0, "message": e.1});
        }
        _ => {}
    }

    return match auto_save_all_users(&mappings, &users) {
        Ok(_) => json!({"status": 200, "message": "User successfully updated!"}),
        Err(e) => json!({"status": 500, "message": e}),
    };
}
