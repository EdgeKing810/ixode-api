use rocket::patch;
use rocket::serde::json::{json, Json, Value};

use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::project::{auto_fetch_all_projects, auto_save_all_projects};
use crate::utils::user::auto_fetch_all_users;

use super::add_member::MemberProjectInput;

#[patch("/member/remove", format = "json", data = "<data>")]
pub async fn main(data: Json<MemberProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let target_uid = &data.target_uid;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to remove users to this Project"});
    }

    match Project::remove_member(&mut all_projects, project_id, target_uid) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "project_remove_member",
        format!(
            "The user usr[{}] was removed from the project pro[{}] by usr[{}]",
            target_uid, project_id, uid
        ),
        format!("/project/{}", project_id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => {
            return json!({"status": 200, "message": "User successfully removed from Project!"})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
