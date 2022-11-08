use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::{auto_fetch_all_users, auto_save_all_users};

#[delete("/delete?<uid>&<target_uid>")]
pub async fn main(token: Token, uid: Option<&str>, target_uid: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_target_uid = match target_uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No target_uid provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
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

    let current_user = User::get(&users, &passed_uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let target_user = match User::get(&users, &passed_target_uid) {
        Ok(u) => u,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };
    if target_user.role == Role::ROOT {
        return json!({"status": 403, "message": "Error: Cannot delete this User"});
    }

    match User::delete(&mut users, &passed_target_uid) {
        Err(e) => return json!({"error": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "user_delete",
        format!(
            "The user <{}> was deleted by usr[{}]",
            target_user.username, &passed_uid
        ),
        format!("/users"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    return match auto_save_all_users(&mappings, &users) {
        Ok(_) => json!({"status": 200, "message": "User successfully deleted!"}),
        Err(e) => json!({"status": 500, "message": e}),
    };
}
