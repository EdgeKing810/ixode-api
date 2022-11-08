use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::collection::{auto_fetch_all_collections, auto_save_all_collections};
use crate::utils::data::{auto_fetch_all_data, auto_save_all_data};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::project::{auto_fetch_all_projects, auto_save_all_projects};
use crate::utils::route::{auto_fetch_all_routes, auto_save_all_routes};
use crate::utils::user::auto_fetch_all_users;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum UpdateType {
    ID,
    NAME,
    DESCRIPTION,
    APIPATH,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateProjectInput {
    uid: String,
    project_id: String,
    change: UpdateType,
    data: String,
}

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<UpdateProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let change = &data.change;
    let data = &data.data;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
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

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(c) => c,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
        }
    };

    let mut all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(r) => r,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let mut all_project_data = Vec::<Data>::new();
    for col in all_collections.iter() {
        if col.project_id == *project_id {
            let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &col.id) {
                Ok(u) => u,
                _ => {
                    return json!({"status": 500, "message": "Error: Failed fetching data"});
                }
            };
            all_project_data.append(&mut all_data);
        }
    }

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
        return json!({"status": 403, "message": "Error: Not authorized to modify this Project"});
    }

    match match change.clone() {
        UpdateType::ID => Project::update_id(&mut all_projects, project_id, data),
        UpdateType::NAME => Project::update_name(&mut all_projects, project_id, data),
        UpdateType::DESCRIPTION => Project::update_description(&mut all_projects, project_id, data),
        UpdateType::APIPATH => Project::update_api_path(&mut all_projects, project_id, data),
    } {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if change.clone() == &UpdateType::ID {
        Data::bulk_update_project_id(&mut all_project_data, project_id, data);

        for route in all_routes.clone().iter() {
            match RouteComponent::update_project_id(&mut all_routes, &route.route_id, data) {
                Err(e) => return json!({"status": e.0, "message": e.1}),
                _ => {}
            }
        }

        for col in all_collections.clone().iter() {
            if col.project_id == *project_id {
                match Collection::update_project_id(&mut all_collections, &col.id, data) {
                    Err(e) => return json!({"status": e.0, "message": e.1}),
                    _ => {}
                }

                let current_data = all_project_data
                    .iter()
                    .filter(|d| d.collection_id == col.id)
                    .cloned()
                    .collect::<Vec<Data>>();

                match auto_save_all_data(&mappings, &data, &col.id, &current_data) {
                    Ok(_) => {}
                    Err(e) => {
                        return json!({"status": 500, "message": e});
                    }
                }
            }
        }

        match auto_save_all_routes(&data, &all_routes) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        match auto_save_all_collections(&mappings, &all_collections) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_id",
            format!(
                "The id of the project pro[{}] was updated from <{}> to <{}> by usr[{}]",
                data, project_id, data, uid
            ),
            format!("/project/{}", data),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::NAME {
        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_name",
            format!(
                "The name of the project pro[{}] was updated from <{}> to <{}> by usr[{}]",
                project_id, project.name, data, uid
            ),
            format!("/project/{}", project_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::DESCRIPTION {
        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_description",
            format!(
                "The description of the project pro[{}] was updated by usr[{}]",
                project_id, uid
            ),
            format!("/project/{}", project_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::APIPATH {
        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_api_path",
            format!(
                "The api_path of the project pro[{}] was updated from <{}> to <{}> by usr[{}]",
                project_id, project.api_path, data, uid
            ),
            format!("/project/{}", project_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "Project successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
