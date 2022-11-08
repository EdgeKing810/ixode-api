use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::custom_structures::CustomStructure;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    collection::auto_fetch_all_collections, collection::auto_save_all_collections,
    data::auto_fetch_all_data, data::auto_save_all_data, event::auto_create_event,
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCustomStructureInput {
    uid: String,
    project_id: String,
    collection_id: String,
    custom_structure_id: String,
    custom_structure: CustomStructure,
}

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<UpdateCustomStructureInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let custom_structure_id = &data.custom_structure_id;
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
        return json!({"status": 403, "message": "Error: Not authorized to update CustomStructures in this Collection"});
    }

    let col = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    if !CustomStructure::exist(&mut col.custom_structures.clone(), custom_structure_id) {
        return json!({"status": 404, "message": "Error: No CustomStructure with this custom_structure_id found"});
    }

    match Collection::update_custom_structure(
        &mut all_collections,
        collection_id,
        custom_structure_id,
        custom_structure.clone(),
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if custom_structure_id != &custom_structure.id {
        let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
            Ok(u) => u,
            _ => {
                return json!({"status": 500, "message": "Error: Failed fetching data"});
            }
        };

        match Data::bulk_update_custom_structure_id(
            &mut all_data,
            project_id,
            collection_id,
            custom_structure_id,
            &custom_structure.id,
        ) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        match auto_save_all_data(&mappings, project_id, collection_id, &all_data) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "custom_structure_update",
        format!(
            "A custom structure with id <{}> under pro[{}]/col[{}] was updated by usr[{}]",
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
        Ok(_) => return json!({"status": 200, "message": "Custom Structure successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
