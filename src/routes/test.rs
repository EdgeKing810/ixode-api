use regex::Regex;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post};

use crate::components::user::User;
use crate::middlewares::token::Token;
use crate::utils::{auto_fetch_all_mappings, auto_fetch_all_users};

#[get("/<name>/<age>")]
pub fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInput {
    auth_data: String,
    password: String,
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginInput>, token: Token) -> Value {
    let auth_data = &data.auth_data;
    let password = &data.password;

    println!("{} {} {}", auth_data, password, token);

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": "500", "message": "Error: Failed fetching users"});
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
            "status": "404",
            "message": format!("Error: Account with this {} not found",
            if is_username {"Usernmae"} else {"Email Address"})
        });
    }

    let user = match if is_username {
        User::login_username(&users, &auth_data, &password)
    } else {
        User::login_email(&users, &auth_data, &password)
    } {
        Ok(user) => user,
        _ => return json!({"status": "401", "message": "Error: Incorrect Password"}),
    };

    json!({"status": "200", "message": "Works!", "user": user})
}
