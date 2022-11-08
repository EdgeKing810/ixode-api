use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::{auto_fetch_all_users, auto_save_all_users};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RoleInput {
    uid: String,
    target_uid: String,
    role: Role,
}

#[patch("/update/role", format = "json", data = "<data>")]
pub async fn main(data: Json<RoleInput>, token: Token) -> Value {
    let uid = &data.uid;
    let target_uid = &data.target_uid;
    let role = &data.role;

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

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let target_user = match User::get(&users, target_uid) {
        Ok(u) => u,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };
    if target_user.role == Role::ROOT {
        return json!({"status": 403, "message": "Error: Cannot change the ROLE of this User"});
    }

    let role_numeric: u32 = match role.clone() {
        Role::ROOT => 0,
        Role::ADMIN => 1,
        Role::AUTHOR => 2,
        _ => 3,
    };

    let old_role_str = match target_user.role.clone() {
        Role::ROOT => "ROOT",
        Role::ADMIN => "ADMIN",
        Role::AUTHOR => "AUTHOR",
        _ => "VIEWER",
    };

    let new_role_str = match role.clone() {
        Role::ROOT => "ROOT",
        Role::ADMIN => "ADMIN",
        Role::AUTHOR => "AUTHOR",
        _ => "VIEWER",
    };

    match User::update_role(&mut users, target_uid, role_numeric) {
        Err(e) => return json!({"error": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "user_role_update",
        format!(
            "The role of usr[{}] was changed from <{}> to <{}> by usr[{}]",
            target_uid, old_role_str, new_role_str, uid
        ),
        format!("/users"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    return match auto_save_all_users(&mappings, &users) {
        Ok(_) => json!({"status": 200, "message": "User role successfully updated!"}),
        Err(e) => json!({"status": 500, "message": e}),
    };
}
