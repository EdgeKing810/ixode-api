use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::project::Project;
use crate::components::structures::Structure;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_fetch_all_collections, auto_fetch_all_mappings, auto_fetch_all_projects,
    auto_fetch_all_users, auto_save_all_collections,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddStructureInput {
    uid: String,
    project_id: String,
    collection_id: String,
    structure: Structure,
}

#[post("/add", format = "json", data = "<data>")]
pub async fn add(data: Json<AddStructureInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
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
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to add Structures to this Collection"});
    }

    match Collection::add_structure(&mut all_collections, collection_id, structure.clone()) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Structure successfully added!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateStructureInput {
    uid: String,
    project_id: String,
    collection_id: String,
    structure_id: String,
    structure: Structure,
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<UpdateStructureInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let structure_id = &data.structure_id;
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
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to update Structures in this Collection"});
    }

    let col = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    if !Structure::exist(&mut col.structures.clone(), structure_id) {
        return json!({"status": 404, "message": "Error: No Structure with this structure_id found"});
    }

    match Collection::update_structure(
        &mut all_collections,
        collection_id,
        structure_id,
        structure.clone(),
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Structure successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteStructureInput {
    uid: String,
    project_id: String,
    collection_id: String,
    structure_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteStructureInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let structure_id = &data.structure_id;

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
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to delete Structures in this Collection"});
    }

    let col = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    if !Structure::exist(&mut col.structures.clone(), structure_id) {
        return json!({"status": 404, "message": "Error: No Structure with this structure_id found"});
    }

    match Collection::remove_structure(&mut all_collections, collection_id, structure_id) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Structure successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
