use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::collection::auto_save_all_collections;
use crate::utils::data::{auto_fetch_all_data, auto_save_all_data};
use crate::utils::event::auto_create_event;
use crate::utils::route::{auto_fetch_all_routes, auto_save_all_routes};
use crate::utils::{
    collection::auto_fetch_all_collections, mapping::auto_fetch_all_mappings,
    project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum UpdateType {
    ID,
    NAME,
    DESCRIPTION,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCollectionInput {
    uid: String,
    project_id: String,
    collection_id: String,
    change: UpdateType,
    data: String,
}

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<UpdateCollectionInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let change = &data.change;
    let data = &data.data;

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
        return json!({"status": 403, "message": "Error: Not authorized to update Collections in this Project"});
    }

    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    let mut all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(r) => r,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let current_col = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    match match change.clone() {
        UpdateType::ID => Collection::update_id(&mut all_collections, collection_id, data),
        UpdateType::NAME => Collection::update_name(&mut all_collections, collection_id, data),
        UpdateType::DESCRIPTION => {
            Collection::update_description(&mut all_collections, collection_id, data)
        }
    } {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if change.clone() == &UpdateType::ID {
        Data::bulk_update_collection_id(&mut all_data, collection_id, data);

        match RouteComponent::bulk_update_collection_id(&mut all_routes, collection_id, data) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        match auto_save_all_data(&mappings, project_id, &data, &all_data) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        match auto_save_all_routes(&project_id, &all_routes) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "collection_update_id",
            format!(
                "The id of the collection col[{}] under pro[{}] was updated from <{}> to <{}> by usr[{}]",
                data, project_id, collection_id, data, uid
            ),
            format!("/project/{}/collection/{}", project_id, data),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::NAME {
        if let Err(e) = auto_create_event(
            &mappings,
            "collection_update_name",
            format!(
                "The name of the collection col[{}] under pro[{}] was updated from <{}> to <{}> by usr[{}]",
                collection_id, project_id, current_col.name, data, uid
            ),
            format!("/project/{}/collection/{}", project_id, collection_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::DESCRIPTION {
        if let Err(e) = auto_create_event(
            &mappings,
            "collection_update_description",
            format!(
                "The description of the collection col[{}] under pro[{}] was updated by usr[{}]",
                collection_id, project_id, uid
            ),
            format!("/project/{}/collection/{}", project_id, collection_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Collection successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
