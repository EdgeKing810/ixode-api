use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::config::Config;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::config::{auto_fetch_all_configs, auto_save_all_configs};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::auto_fetch_all_users;

#[delete("/delete?<uid>&<key>")]
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

    let current_user = User::get(&users, &passed_uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let exists = Config::exist(&all_configs, &passed_key);

    if !exists {
        return json!({"status": 404, "message": "Error: No Config with this Key found"});
    }

    match Config::delete(&mut all_configs, &passed_key) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "config_delete",
        format!(
            "The config <{}> was deleted by usr[{}]",
            &passed_key, &passed_uid
        ),
        format!("/configs"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_configs(&mappings, &all_configs) {
        Ok(_) => return json!({"status": 200, "message": "Config successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
