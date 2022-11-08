use regex::Regex;
use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::user::User;
use crate::middlewares::token::create_jwt;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::auto_fetch_all_users;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInput {
    auth_data: String,
    password: String,
}

#[post("/login", format = "json", data = "<data>")]
pub fn main(data: Json<LoginInput>) -> Value {
    let auth_data = &data.auth_data;
    let password = &data.password;

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let mut is_username = true;

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if email_regex.is_match(auth_data) {
        is_username = false;
    }

    if (is_username && !User::exist_username(&users, &auth_data))
        || (!is_username && !User::exist_email(&users, &auth_data))
    {
        return json!({
            "status": 404,
            "message": format!("Error: Account with this {} not found",
            if is_username {"Username"} else {"Email Address"})
        });
    }

    let user = match if is_username {
        User::login_username(&users, &auth_data, &password)
    } else {
        User::login_email(&users, &auth_data, &password)
    } {
        Ok(user) => user,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    let jwt = match create_jwt(&mappings, user.id.clone()) {
        Ok(token) => token,
        Err(e) => return json!({"status": 500, "message": e}),
    };

    json!({"status": 200, "message": "Login Successful!", "user": user, "uid": user.id, "jwt": jwt})
}
