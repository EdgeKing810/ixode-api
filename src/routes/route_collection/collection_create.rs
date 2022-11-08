use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CollectionInput {
    id: String,
    project_id: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateCollectionInput {
    uid: String,
    collection: CollectionInput,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn main(data: Json<CreateCollectionInput>, token: Token) -> Value {
    let uid = &data.uid;
    let collection = &data.collection;

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

    let project = match Project::get(&all_projects, &collection.project_id) {
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
        return json!({"status": 403, "message": "Error: Not authorized to create Collections in this Project"});
    }

    match Collection::create(
        &mut all_collections,
        &collection.id,
        &collection.project_id,
        &collection.name,
        &collection.description,
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "collection_create",
        format!(
            "A new collection named col[{}] was created under pro[{}] by usr[{}]",
            collection.id, collection.project_id, uid
        ),
        format!(
            "/project/{}/collection/{}",
            collection.project_id, collection.id
        ),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Collection successfully created!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
