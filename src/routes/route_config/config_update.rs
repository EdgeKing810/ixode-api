use rocket::patch;
use rocket::serde::json::{json, Json, Value};

use crate::components::config::Config;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::config::{auto_fetch_all_configs, auto_save_all_configs};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::auto_fetch_all_users;

use super::add::AddConfigInput;

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<AddConfigInput>, token: Token) -> Value {
    let uid = &data.uid;
    let key = &data.key;
    let value = &data.value;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_configs = match auto_fetch_all_configs(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching configs"});
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let exists = Config::exist(&all_configs, key);

    if !exists {
        return json!({"status": 404, "message": "Error: No Config with this Key found"});
    }

    match Config::update_value(&mut all_configs, key, value) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "config_update",
        format!("The config <{}> was updated by usr[{}]", key, uid),
        format!("/configs"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_configs(&mappings, &all_configs) {
        Ok(_) => return json!({"status": 200, "message": "Config successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
