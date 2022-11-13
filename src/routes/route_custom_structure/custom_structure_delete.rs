use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::collection::Collection;
use crate::components::custom_structure::CustomStructure;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    collection::auto_fetch_all_collections, collection::auto_save_all_collections,
    event::auto_create_event, mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects,
    user::auto_fetch_all_users,
};

#[delete("/delete?<uid>&<project_id>&<collection_id>&<custom_structure_id>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    project_id: Option<&str>,
    collection_id: Option<&str>,
    custom_structure_id: Option<&str>,
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

    let passed_custom_structure_id = match custom_structure_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No custom_structure_id provided"}),
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
        return json!({"status": 403, "message": "Error: Not authorized to delete CustomStructures in this Collection"});
    }

    let col = match Collection::get(&all_collections, &passed_project_id, &passed_collection_id) {
        Ok(col) => col,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    if !CustomStructure::exist(
        &mut col.custom_structures.clone(),
        &passed_custom_structure_id,
    ) {
        return json!({"status": 404, "message": "Error: No CustomStructure with this custom_structure_id found"});
    }

    match Collection::remove_custom_structure(
        &mut all_collections,
        &passed_collection_id,
        &passed_custom_structure_id,
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "custom_structure_delete",
        format!(
            "A custom structure with id <{}> under pro[{}]/col[{}] was deleted by usr[{}]",
            passed_custom_structure_id, passed_project_id, passed_collection_id, passed_uid
        ),
        format!(
            "/project/{}/collection/{}",
            passed_project_id, passed_collection_id
        ),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Custom Structure successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
