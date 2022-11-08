use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    collection::auto_fetch_all_collections, data::auto_fetch_all_data,
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

use crate::utils::x::convertors::convert_data_to_rawpair::data_to_rawpair;

#[get("/fetch/one?<uid>&<project_id>&<collection_id>&<data_id>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    project_id: Option<&str>,
    collection_id: Option<&str>,
    data_id: Option<&str>,
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

    let passed_data_id = match data_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No data_id provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, &passed_uid).unwrap();

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

    let project = match Project::get(&all_projects, &passed_project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let collection = match Collection::get(
        &all_collections,
        &passed_project_id,
        &passed_collection_id,
    ) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        for member in members {
            if member.to_lowercase() == passed_uid {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to view Data for this Collection"});
    }

    let all_data = match auto_fetch_all_data(&mappings, &passed_project_id, &passed_collection_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    let current_data = match Data::get(
        &all_data,
        &passed_project_id,
        &passed_collection_id,
        &passed_data_id,
    ) {
        Ok(d) => d,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Data with this data_id found"})
        }
    };

    let raw_pair = match data_to_rawpair(&current_data, &collection) {
        Ok(rp) => rp,
        Err(e) => {
            return json!({"status": e.0, "message": e.1});
        }
    };

    return json!({"status": 200, "message": "Data successfully fetched!", "pair": raw_pair, "data_id": data_id});
}
