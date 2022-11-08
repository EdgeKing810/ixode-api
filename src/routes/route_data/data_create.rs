use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::project::Project;
use crate::components::raw_pair::RawPair;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    collection::auto_fetch_all_collections, data::auto_fetch_all_data, data::auto_save_all_data,
    event::auto_create_event, mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects,
    user::auto_fetch_all_users,
};

use crate::utils::x::convertors::convert_rawpair_to_data::rawpair_to_data;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDataInput {
    uid: String,
    project_id: String,
    collection_id: String,
    raw_pair: RawPair,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn main(data: Json<CreateDataInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let raw_pair = &data.raw_pair;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let collection = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::AUTHOR {
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
        return json!({"status": 403, "message": "Error: Not authorized to create Data for this Collection"});
    }

    let data_id = match rawpair_to_data(&mut all_data, &collection, raw_pair, false) {
        Ok(id) => id,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    if let Err(e) = auto_create_event(
        &mappings,
        "data_create",
        format!(
            "A new data with id <{}> was created under pro[{}]/col[{}] by usr[{}]",
            data_id, project_id, collection_id, uid
        ),
        format!("/data/p/{}/c/{}/d/v/{}", project_id, collection_id, data_id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_data(&mappings, &project_id, &collection_id, &all_data) {
        Ok(_) => {
            return json!({"status": 200, "message": "Data successfully created!", "data_id": data_id})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
