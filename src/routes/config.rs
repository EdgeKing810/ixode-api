use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::config::Config;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_create_event, auto_fetch_all_configs, auto_fetch_all_mappings, auto_fetch_all_users,
    auto_save_all_configs,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NormalInput {
    uid: String,
}

#[post("/fetch?<limit>&<offset>", format = "json", data = "<data>")]
pub async fn fetch_all(
    data: Json<NormalInput>,
    token: Token,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Value {
    let uid = &data.uid;

    let passed_limit = match limit {
        Some(x) => x,
        None => 0,
    };
    let passed_offset = match offset {
        Some(x) => x,
        None => 0,
    };

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

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_configs = match auto_fetch_all_configs(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching configs"});
        }
    };
    let amount = all_configs.len();
    let processed_configs = paginate(all_configs, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Configs successfully fetched!", "configs": processed_configs, "amount": amount});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConfigFetchInput {
    uid: String,
    key: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<ConfigFetchInput>, token: Token) -> Value {
    let uid = &data.uid;
    let key = &data.key;

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

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_configs = match auto_fetch_all_configs(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching configs"});
        }
    };

    if !Config::exist(&all_configs, key) {
        return json!({"status": 404, "message": "Error: No Config with this Key found"});
    }

    let value = Config::get_value(&all_configs, key);

    return json!({"status": 200, "message": "Config successfully fetched!", "value": value});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddConfigInput {
    uid: String,
    key: String,
    value: String,
}

#[post("/add", format = "json", data = "<data>")]
pub async fn add(data: Json<AddConfigInput>, token: Token) -> Value {
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

    match if exists {
        Config::update_value(&mut all_configs, key, value)
    } else {
        Config::create(&mut all_configs, key, value)
    } {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "config_create",
        format!("The config <{}> was created by usr[{}]", key, uid),
        format!("/configs"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_configs(&mappings, &all_configs) {
        Ok(_) => return json!({"status": 200, "message": "Config successfully created!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<AddConfigInput>, token: Token) -> Value {
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteConfigInput {
    uid: String,
    key: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteConfigInput>, token: Token) -> Value {
    let uid = &data.uid;
    let key = &data.key;

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

    match Config::delete(&mut all_configs, key) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "config_delete",
        format!("The config <{}> was deleted by usr[{}]", key, uid),
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
