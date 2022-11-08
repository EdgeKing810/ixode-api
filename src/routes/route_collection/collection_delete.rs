use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::collection::Collection;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::collection::auto_save_all_collections;
use crate::utils::event::auto_create_event;
use crate::utils::{
    collection::auto_fetch_all_collections, mapping::auto_fetch_all_mappings,
    project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

#[delete("/delete?<uid>&<project_id>&<collection_id>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    project_id: Option<&str>,
    collection_id: Option<&str>,
) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_project_id = match project_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No project_id provided"}),
    };

    let passed_collection_id = match collection_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No collection_id provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
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

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

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
        return json!({"status": 403, "message": "Error: Not authorized to delete Collections in this Project"});
    }

    let current_col =
        match Collection::get(&all_collections, &passed_project_id, &passed_collection_id) {
            Ok(col) => col,
            Err(e) => return json!({"status": e.0, "message": e.1}),
        };

    match Collection::delete(&mut all_collections, &passed_collection_id) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "collection_delete",
        format!(
            "The collection <{}> under pro[{}] was deleted by usr[{}]",
            current_col.name, &passed_project_id, &passed_uid
        ),
        format!("/project/{}", &passed_project_id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Collection successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
