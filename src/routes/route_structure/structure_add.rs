use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::custom_structures::CustomStructure;
use crate::components::data::Data;
use crate::components::datapair::DataPair;
use crate::components::encryption::EncryptionKey;
use crate::components::project::Project;
use crate::components::structures::Structure;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    collection::auto_fetch_all_collections, collection::auto_save_all_collections,
    data::auto_fetch_all_data, data::auto_save_all_data, event::auto_create_event,
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddStructureInput {
    uid: String,
    project_id: String,
    collection_id: String,
    custom_structure_id: String,
    structure: Structure,
}

#[post("/add", format = "json", data = "<data>")]
pub async fn main(data: Json<AddStructureInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let custom_structure_id = &data.custom_structure_id;
    let structure = &data.structure;

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
        return json!({"status": 403, "message": "Error: Not authorized to add Structures to this Collection"});
    }

    if custom_structure_id.trim().len() <= 0 {
        match Collection::add_structure(&mut all_collections, collection_id, structure.clone()) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "structure_create",
            format!(
                "A new structure with id <{}> was created under pro[{}]/col[{}] by usr[{}]",
                structure.id, project_id, collection_id, uid
            ),
            format!("/project/{}/collection/{}", project_id, collection_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else {
        let current_collection = match all_collections.iter_mut().find(|c| c.id == *collection_id) {
            Some(c) => c,
            None => {
                return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
            }
        };

        let mut all_custom_structures = current_collection.custom_structures.clone();
        match CustomStructure::add_structure(
            &mut all_custom_structures,
            custom_structure_id,
            structure.clone(),
        ) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        match Collection::set_custom_structures(
            &mut all_collections,
            collection_id,
            all_custom_structures,
        ) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "structure_create_custom",
            format!(
                "A new structure with id <{}> was created under pro[{}]/col[{}]/<{}> by usr[{}]",
                structure.id, project_id, collection_id, custom_structure_id, uid
            ),
            format!(
                "/project/{}/collection/{}/custom/{}",
                project_id, collection_id, custom_structure_id
            ),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    for data in all_data.clone() {
        let new_pair = DataPair {
            id: EncryptionKey::generate_uuid(16),
            structure_id: structure.id.clone(),
            custom_structure_id: custom_structure_id.clone(),
            dtype: Structure::from_stype(structure.stype.clone()),
            value: String::new(),
        };

        match Data::add_pair(&mut all_data, &data.id, new_pair) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": e.0, "message": e.1});
            }
        }
    }

    match auto_save_all_data(&mappings, project_id, collection_id, &all_data) {
        Ok(_) => {}
        Err(e) => {
            return json!({"status": 500, "message": e});
        }
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Structure successfully added!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
