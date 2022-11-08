use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::collection::Collection;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::collection::{auto_fetch_all_collections, auto_save_all_collections};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::project::{auto_fetch_all_projects, auto_save_all_projects};
use crate::utils::user::auto_fetch_all_users;

#[delete("/delete?<uid>&<project_id>")]
pub async fn main(token: Token, uid: Option<&str>, project_id: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_project_id = match project_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No project_id provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
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

    let current_user = User::get(&users, &passed_uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let project = match Project::get(&all_projects, &passed_project_id) {
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
                if member.to_lowercase() == passed_uid {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to delete this Project"});
    }

    match Project::delete(&mut all_projects, &passed_project_id) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
        }
    };

    Collection::delete_by_project(&mut all_collections, &passed_project_id);

    match auto_save_all_collections(&mappings, &all_collections) {
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "project_delete",
        format!(
            "The project <{}> was deleted by usr[{}]",
            project.name, &passed_uid
        ),
        format!("/projects"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "Project successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
