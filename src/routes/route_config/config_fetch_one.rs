use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::config::Config;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::config::auto_fetch_all_configs;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::auto_fetch_all_users;

#[get("/fetch/one?<uid>&<key>")]
pub async fn main(token: Token, uid: Option<&str>, key: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_key = match key {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No key provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
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

    let current_user = User::get(&users, &passed_uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_configs = match auto_fetch_all_configs(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching configs"});
        }
    };

    if !Config::exist(&all_configs, &passed_key) {
        return json!({"status": 404, "message": "Error: No Config with this Key found"});
    }

    let value = Config::get_value(&all_configs, &passed_key);

    return json!({"status": 200, "message": "Config successfully fetched!", "value": value});
}
