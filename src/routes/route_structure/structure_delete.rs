use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::collection::Collection;
use crate::components::custom_structures::CustomStructure;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::structures::Structure;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    collection::auto_fetch_all_collections, collection::auto_save_all_collections,
    data::auto_fetch_all_data, data::auto_save_all_data, event::auto_create_event,
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

#[delete("/delete?<uid>&<project_id>&<collection_id>&<structure_id>&<custom_structure_id>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    project_id: Option<&str>,
    collection_id: Option<&str>,
    structure_id: Option<&str>,
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

    let passed_structure_id = match structure_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No structure_id provided"}),
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
        return json!({"status": 403, "message": "Error: Not authorized to delete Structures in this Collection"});
    }

    let col = match Collection::get(&all_collections, &passed_project_id, &passed_collection_id) {
        Ok(col) => col,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    if passed_custom_structure_id.trim().len() <= 0 {
        if !Structure::exist(&mut col.structures.clone(), &passed_structure_id) {
            return json!({"status": 404, "message": "Error: No Structure with this structure_id found"});
        }

        match Collection::remove_structure(
            &mut all_collections,
            &passed_collection_id,
            &passed_structure_id,
        ) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "structure_delete",
            format!(
                "A structure with id <{}> under pro[{}]/col[{}] was deleted by usr[{}]",
                passed_structure_id, passed_project_id, passed_collection_id, passed_uid
            ),
            format!(
                "/project/{}/collection/{}",
                passed_project_id, passed_collection_id
            ),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else {
        let mut all_custom_structures = col.custom_structures.clone();
        let current_custom_structure = match all_custom_structures
            .iter_mut()
            .find(|cs| cs.id == passed_custom_structure_id)
        {
            Some(c) => c,
            None => {
                return json!({"status": 404, "message": "Error: No Custom Structure with this custom_structure_id found"})
            }
        };

        if !Structure::exist(
            &mut current_custom_structure.structures.clone(),
            &passed_structure_id,
        ) {
            return json!({"status": 404, "message": "Error: No Structure with this structure_id found"});
        }

        match CustomStructure::remove_structure(
            &mut all_custom_structures,
            &passed_custom_structure_id,
            &passed_structure_id,
        ) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        match Collection::set_custom_structures(
            &mut all_collections,
            &passed_collection_id,
            all_custom_structures,
        ) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "structure_delete_custom",
            format!(
                "A structure with id <{}> under pro[{}]/col[{}]/<{}> was deleted by usr[{}]",
                passed_structure_id,
                passed_project_id,
                passed_collection_id,
                passed_custom_structure_id,
                passed_uid
            ),
            format!(
                "/project/{}/collection/{}/custom/{}",
                passed_project_id, passed_collection_id, passed_custom_structure_id
            ),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    let mut all_data =
        match auto_fetch_all_data(&mappings, &passed_project_id, &passed_collection_id) {
            Ok(u) => u,
            _ => {
                return json!({"status": 500, "message": "Error: Failed fetching data"});
            }
        };

    for data in all_data.clone() {
        let pair_id = match data.pairs.iter().find(|dp| {
            dp.structure_id == passed_structure_id
                && dp.custom_structure_id == passed_custom_structure_id
        }) {
            Some(x) => x.id.clone(),
            None => String::new(),
        };

        if pair_id.len() > 0 {
            match Data::remove_pair(&mut all_data, &data.id, &pair_id) {
                Err(e) => return json!({"status": e.0, "message": e.1}),
                _ => {}
            }
        }
    }

    match auto_save_all_data(
        &mappings,
        &passed_project_id,
        &passed_collection_id,
        &all_data,
    ) {
        Ok(_) => {}
        Err(e) => {
            return json!({"status": 500, "message": e});
        }
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Structure successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
