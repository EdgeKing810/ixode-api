use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddCustomStructureInput {
    uid: String,
    project_id: String,
    collection_id: String,
    custom_structure: CustomStructure,
}

#[post("/add", format = "json", data = "<data>")]
pub async fn main(data: Json<AddCustomStructureInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let cs = &data.custom_structure;

    let custom_structure = CustomStructure {
        id: cs.id.clone(),
        name: cs.name.clone(),
        description: cs.description.clone(),
        structures: vec![],
    }; // Prevent skipping proper validation to be applied to relevant 'Data'

    match verify_jwt(uid.clone(), token.0).await {
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

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

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
        return json!({"status": 403, "message": "Error: Not authorized to add CustomStructures to this Collection"});
    }

    match Collection::add_custom_structure(
        &mut all_collections,
        collection_id,
        custom_structure.clone(),
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "custom_structure_create",
        format!(
            "A new custom structure with id <{}> was created under pro[{}]/col[{}] by usr[{}]",
            custom_structure.id, project_id, collection_id, uid
        ),
        format!(
            "/project/{}/collection/{}/custom/{}",
            project_id, collection_id, custom_structure.id
        ),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Custom Structure successfully added!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
